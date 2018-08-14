extern crate gears;
#[macro_use]
extern crate log;
extern crate clap;
extern crate rustyline;

extern crate actix;
extern crate actix_web;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate juniper;
extern crate futures;

use clap::{Arg, App, SubCommand};
use std::io::{self, Read};
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use gears::structure::model::ModelDocument;

extern crate env_logger;

mod app;
use app::{AppState, Format};

mod shell;
mod server;
mod model_schema;

fn load_model(path: &str) -> ModelDocument {
    let model = gears::util::fs::model_from_fs(path).unwrap();
    model
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();
    buffer
}


fn write_file(filename: &str, data: &str) -> () {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => {
            error!("couldn't create {}: {}", display, why.description());
            panic!("couldn't create {}: {}", display, why.description());
        }
        Ok(file) => file,
    };

    match file.write_all(data.as_bytes()) {
        Err(why) => {
            error!("couldn't write to {}: {}", display, why.description());
            panic!("couldn't write to {}: {}", display, why.description());
        }
        Ok(_) => debug!("successfully wrote to {}", display),
    }
}


fn add_project_files(path: &str) -> () {

    write_file(
        &(format!("{}/.gitignore", path)),
        r#"**/*tmp
**/*log
**/*.bk
**/*.swp
**/*.swo
history.gears-shell
local.json
out/
"#,
    );

    write_file(
        &(format!("{}/README.md", path)),
        r#"
# gears project

This project is suitable for version control.

It is recommended to initialize this project with `git`, though any VCS will do.

    git init
    git add .
    git commit -m "Initial commit"

## Help

Have the `gears-cli` tool installed. See 
[http://github.com/gears-project/gears-cli](http://github.com/gears-project/gears-cli)

For general information, visit the project hub at
[http://github.com/gears-project/](http://github.com/gears-project/)

    gears-cli --help

## Build

    gears-cli build

## Interactive shell

    gears-cli shell
    << Running gears-shell
	>> list xflow
	XFlow: ID Uuid("606dc85d-9daf-4045-8b85-0c7ccb667c63") - "zork"
	>> generate xflow my_first_xflow
	XFlow: ID Uuid("5e0d1a30-9c48-489c-af2d-a34054c98316") - "my_first_xflow"
	>> generate page my_first_page
	Page: ID Uuid("fc016992-95ad-49aa-9cb4-9814ce803d9a") - "my_first_page"
	>> generate translation es_ES
	>> list translation
	Translation: ID Uuid("0cab532f-3c5c-49a7-89c0-9132e14039a8") - "default" - "en_US"
	Translation: ID Uuid("5f64834b-bfb4-4075-966d-0d8a4cfe6232") - "default" - "es_ES"
    >> sync

When using the interactive shell to make changes, remember that changes are **ONLY SAVED AFTER
ISSUING A `sync` COMMAND**.

"#,
    );

}

fn main() {
    let _ = env_logger::init();

    let matches = App::new("gears-cli")
        .version("0.1.11")
        .author("Michiel Kalkman <michiel@nosuchtype.com")
        .about("CLI tool for working with gears-project models")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .value_name("path")
                .default_value(".")
                .help("Sets a project path")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output_path")
                .long("output-path")
                .value_name("output_path")
                .default_value("out")
                .help("Sets the output path")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("locale")
                .short("l")
                .long("locale")
                .value_name("locale")
                .default_value("en_US")
                .help("Set the project locale")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("input_format")
                .long("input-format")
                .value_name("input_format")
                .possible_values(&["json", "yaml"])
                .default_value("json")
                .help("Sets the input format")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output_format")
                .long("output-format")
                .value_name("output_format")
                .possible_values(&["json", "yaml"])
                .default_value("json")
                .help("Sets the output format")
                .takes_value(true),
        )
        .arg(Arg::with_name("v").short("v").multiple(true).help(
            "Sets the level of verbosity",
        ))
        .subcommand(SubCommand::with_name("shell").about(
            "Run an interactive shell",
        ))
        .subcommand(SubCommand::with_name("init").about(
            "Initialize a new project",
        ))
        .subcommand(SubCommand::with_name("export").about(
            "Export an existing project",
        ))
        .subcommand(SubCommand::with_name("import").about(
            "Import an existing project",
        ))
        .subcommand(SubCommand::with_name("transform").about(
            "Transform an existing project",
        ))
        .subcommand(SubCommand::with_name("validate").about(
            "Validate an existing project",
        ))
        .subcommand(SubCommand::with_name("build").about(
            "Build project artifacts",
        ))
        .subcommand(SubCommand::with_name("serve").about(
            "Run a web UI for project",
        ))
        .get_matches();


    let config = matches.value_of("config").unwrap_or("project.conf");
    let path = matches.value_of("path").unwrap_or(".");
    let output_path = matches.value_of("output_path").unwrap_or(".");

    let input_format = if matches.value_of("input_format").unwrap_or("json") == "yaml" {
        Format::YAML
    } else {
        Format::JSON
    };

    let output_format = if matches.value_of("output_format").unwrap_or("json") == "yaml" {
        Format::YAML
    } else {
        Format::JSON
    };

    let locale = matches.value_of("locale").unwrap_or("en_US");

    let mut appstate = AppState {
        locale: locale.to_string(),
        path_config: config.to_string(),
        path_in: path.to_string(),
        path_out: output_path.to_string(),
        format_in: input_format.clone(),
        format_out: output_format.clone(),
    };

    match matches.subcommand_name() {
        Some("init") => subcommand_init(&appstate),
        Some("shell") => subcommand_shell(&appstate),
        Some("export") => subcommand_export(&mut appstate),
        Some("import") => subcommand_import(&mut appstate),
        Some("transform") => subcommand_transform(&appstate),
        Some("validate") => subcommand_validate(&appstate),
        Some("build") => subcommand_build(&appstate),
        Some("serve") => subcommand_serve(&appstate),
        None => println!("No subcommand was used"),
        _ => println!("Some other subcommand was used"),
    }
}

fn subcommand_init(appstate: &AppState) -> () {
    info!("init: in directory {}", appstate.path_in);
    let _ = gears::util::fs::init_new_model_dir(&appstate.path_in);
    add_project_files(&appstate.path_in);
}

fn subcommand_shell(appstate: &AppState) -> () {
    info!("shell: in directory {}", appstate.path_in);
    let mut model = load_model(&appstate.path_in);
    shell::shell(&mut model, &appstate);
}

fn subcommand_validate(appstate: &AppState) -> () {
    info!("validate: model in '{}'", appstate.path_in);
    let model = load_model(&appstate.path_in);
    let path_sep = ";".to_owned();
    let errors = gears::validation::common::validate_model(&model);

    if errors.len() > 0 {
        for error in &errors {
            println!(
                "Error '{}' - Path '{}'",
                error.message,
                error.paths.join(&path_sep)
            );
        }
    } else {
        println!("Model '{}' validates OK", model.id);
    }
}

fn subcommand_transform(appstate: &AppState) -> () {
    let buffer = read_stdin();

    let model = match appstate.format_in {
        Format::YAML => gears::structure::model::ModelDocument::from_yaml(&buffer),
        Format::JSON => gears::structure::model::ModelDocument::from_json(&buffer),
    };

    match appstate.format_out {
        Format::YAML => println!("{}", model.to_yaml()),
        Format::JSON => println!("{}", model.to_json()),
    }
}

fn subcommand_build(appstate: &AppState) -> () {
    info!(
        "build: model in '{}', building assets in '{}'",
        appstate.path_in,
        appstate.path_out
    );

    let mut model = load_model(&appstate.path_in);
    model.pad_all_translations();
    let model_locale = model.as_locale(&appstate.locale).unwrap();

    let _ = gears::util::fs::build_to_react_app(&model_locale, &appstate.path_out);

}

fn subcommand_serve(appstate: &AppState) -> () {
    info!(
        "serve: model in '{}'",
        appstate.path_in
    );

    let model = load_model(&appstate.path_in);

    server::serve(&model);
}

fn subcommand_import(appstate: &mut AppState) -> () {
    let buffer = read_stdin();

    let model = match appstate.format_in {
        Format::YAML => gears::structure::model::ModelDocument::from_yaml(&buffer),
        Format::JSON => gears::structure::model::ModelDocument::from_json(&buffer),
    };

    let _ = gears::util::fs::model_to_fs(
        &model.as_locale(&appstate.locale).unwrap(),
        &appstate.path_in,
    ).unwrap();

}

fn subcommand_export(appstate: &mut AppState) -> () {

    let model = gears::util::fs::model_from_fs(&appstate.path_in).unwrap();

    match appstate.format_out {
        Format::YAML => println!("{}", model.as_locale(&appstate.locale).unwrap().to_yaml()),
        Format::JSON => println!("{}", model.as_locale(&appstate.locale).unwrap().to_json()),
    }
}
