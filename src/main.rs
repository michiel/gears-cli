extern crate gears;
#[macro_use]
extern crate log;
extern crate clap;
use clap::{Arg, App, SubCommand, ArgMatches};
use std::io::{self, Read};
extern crate env_logger;

fn load_model(path: &str) -> gears::structure::model::ModelDocument {
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

#[derive(Clone)]
enum Format {
    JSON,
    YAML,
}

struct AppState {
    locale: String,
    path_config: String,
    path_in: String,
    path_out: String,
    format_in: Format,
    format_out: Format,
}

fn main() {
    let _ = env_logger::init();

    let matches = App::new("gears-cli")
        .version("0.1.4")
        .author("Michiel Kalkman <michiel@nosuchtype.com")
        .about("CLI tool for working with gears-project models")
        .arg(Arg::with_name("config")
                 .short("c")
                 .long("config")
                 .value_name("FILE")
                 .help("Sets a custom config file")
                 .takes_value(true))
        .arg(Arg::with_name("path")
                 .short("p")
                 .long("path")
                 .value_name("path")
                 .default_value(".")
                 .help("Sets a project path")
                 .takes_value(true))
        .arg(Arg::with_name("output_path")
                 .long("output-path")
                 .value_name("output_path")
                 .default_value(".")
                 .help("Sets the output path")
                 .takes_value(true))
        .arg(Arg::with_name("locale")
                 .short("l")
                 .long("locale")
                 .value_name("locale")
                 .default_value("en_US")
                 .help("Set the project locale")
                 .takes_value(true))
        .arg(Arg::with_name("input_format")
                 .long("input-format")
                 .value_name("input_format")
                 .possible_values(&["json", "yaml"])
                 .default_value("json")
                 .help("Sets the input format")
                 .takes_value(true))
        .arg(Arg::with_name("output_format")
                 .long("output-format")
                 .value_name("output_format")
                 .possible_values(&["json", "yaml"])
                 .default_value("json")
                 .help("Sets the output format")
                 .takes_value(true))
        .arg(Arg::with_name("v")
                 .short("v")
                 .multiple(true)
                 .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("init").about("Initialize a new project"))
        .subcommand(SubCommand::with_name("export").about("Export an existing project"))
        .subcommand(SubCommand::with_name("import").about("Import an existing project"))
        .subcommand(SubCommand::with_name("transform").about("Transform an existing project"))
        .subcommand(SubCommand::with_name("build").about("Build project artifacts"))
        .subcommand(SubCommand::with_name("validate").about("Validate an existing project"))
        .subcommand(SubCommand::with_name("generate-translation")
                        .about("Add a new locale and translation to a project"))
        .subcommand(SubCommand::with_name("generate")
                        .about("Generate a model component")
                        .arg(Arg::with_name("model_component")
                                 .possible_values(&["xflow", "translation", "page"])))
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
        Some("export") => subcommand_export(&mut appstate),
        Some("import") => subcommand_import(&mut appstate),
        Some("transform") => subcommand_transform(&appstate),
        Some("validate") => subcommand_validate(&appstate),
        Some("build") => subcommand_build(&appstate),
        Some("generate") => subcommand_generate(&appstate, matches.subcommand_matches("generate")),
        None => println!("No subcommand was used"),
        _ => println!("Some other subcommand was used"),
    }
}

fn subcommand_init(appstate: &AppState) -> () {
    info!("init: in directory {}", appstate.path_in);
    let _ = gears::util::fs::init_new_model_dir(&appstate.path_in);
}

fn subcommand_validate(appstate: &AppState) -> () {
    info!("validate: model in '{}'", appstate.path_in);
    let model = load_model(&appstate.path_in);
    let path_sep = ";".to_owned();
    let errors = gears::validation::common::validate_model(&model);

    if errors.len() > 0 {
        for error in &errors {
            println!("Error '{}' - Path '{}'",
                     error.message,
                     error.paths.join(&path_sep));
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
    info!("build: model in '{}', building assets in '{}'",
          appstate.path_in,
          appstate.path_out);

    let mut model = load_model(&appstate.path_in);
    model.pad_all_translations();
    let model_locale = model.as_locale(&appstate.locale).unwrap();

    let _ = gears::util::fs::build_to_react_app(&model_locale, &appstate.path_out);

}

fn subcommand_import(appstate: &mut AppState) -> () {
    let buffer = read_stdin();

    let model = match appstate.format_in {
        Format::YAML => gears::structure::model::ModelDocument::from_yaml(&buffer),
        Format::JSON => gears::structure::model::ModelDocument::from_json(&buffer),
    };

    let _ = gears::util::fs::model_to_fs(&model.as_locale(&appstate.locale).unwrap(),
                                         &appstate.path_in)
        .unwrap();

}

fn subcommand_export(appstate: &mut AppState) -> () {

    let model = gears::util::fs::model_from_fs(&appstate.path_in).unwrap();

    match appstate.format_out {
        Format::YAML => println!("{}", model.as_locale(&appstate.locale).unwrap().to_yaml()),
        Format::JSON => println!("{}", model.as_locale(&appstate.locale).unwrap().to_json()),
    }

}

fn subcommand_generate(appstate: &AppState, matches_option: Option<&ArgMatches>) -> () {
    info!("generate: model in '{}'", appstate.path_in);

    let mut model = load_model(&appstate.path_in);

    match matches_option {
        Some(matches) => {
            match matches.value_of("model_component") {
                Some("xflow") => {
                    info!("generate: xflow");
                    let doc = gears::structure::xflow::XFlowDocument::default();
                    model.doc.xflows.push(doc);
                    let _ = gears::util::fs::model_to_fs(&model, &appstate.path_out).unwrap();
                }
                Some("translation") => {
                    info!("generate: translation");
                    let _ = model.add_locale(&appstate.locale);
                    model.pad_all_translations();
                    let _ = gears::util::fs::model_to_fs(&model, &appstate.path_out).unwrap();
                }
                Some("page") => {
                    info!("generate: page");
                    let doc = gears::structure::page::PageDocument::default();
                    model.doc.pages.push(doc);
                    let _ = gears::util::fs::model_to_fs(&model, &appstate.path_out).unwrap();
                }
                _ => {
                    error!("generate: Incorrect argument");
                }
            }
        }
        None => {
            error!("generate: No matches found for generate task");
        }
    }

}
