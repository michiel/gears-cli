extern crate xflow;
#[macro_use]
extern crate log;
extern crate clap;
use clap::{Arg, App, SubCommand};
use std::io::{self, Read};
extern crate env_logger;

fn load_model(path: &str) -> xflow::structure::model::ModelDocument {
    let model = xflow::util::fs::model_from_fs(path).unwrap();
    model
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();
    buffer
}

enum Format {
    JSON,
    YAML,
}

fn main() {
    let _ = env_logger::init();

    let matches = App::new("gears-cli")
        .version("0.1.0")
        .author("Michiel Kalkman <michiel@nosuchtype.com")
        .about("Does awesome things")
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
                 .help("Sets a project path")
                 .takes_value(true))
        .arg(Arg::with_name("locale")
                 .short("l")
                 .long("locale")
                 .value_name("locale")
                 .help("Set the project locale")
                 .takes_value(true))
        .arg(Arg::with_name("input_format")
                 .long("input-format")
                 .value_name("input_format")
                 .help("Sets the input format")
                 .takes_value(true))
        .arg(Arg::with_name("output_format")
                 .long("output-format")
                 .value_name("output_format")
                 .help("Sets the output format")
                 .takes_value(true))
        .arg(Arg::with_name("v")
                 .short("v")
                 .multiple(true)
                 .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("init")
                        .about("Initialize a new project")
                        .arg(Arg::with_name("dir")
                                 .short("d")
                                 .help("Project directory and name")))
        .subcommand(SubCommand::with_name("export").about("Export an existing project"))
        .subcommand(SubCommand::with_name("import").about("Import an existing project"))
        .subcommand(SubCommand::with_name("transform").about("Transform an existing project"))
        .subcommand(SubCommand::with_name("build").about("Build project artifacts"))
        .subcommand(SubCommand::with_name("validate").about("Validate an existing project"))
        .get_matches();


    let config = matches.value_of("config").unwrap_or("project.conf");
    let path = matches.value_of("path").unwrap_or(".");

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

    match matches.subcommand_name() {
        Some("init") => subcommand_init(&path),
        Some("export") => subcommand_export(&path, &output_format),
        Some("import") => subcommand_import(&path, &input_format),
        Some("transform") => subcommand_transform(&input_format, &output_format),
        Some("validate") => subcommand_validate(&path),
        Some("build") => subcommand_build(&path, &locale),
        None => println!("No subcommand was used"),
        _ => println!("Some other subcommand was used"),
    }

}

fn subcommand_init(path: &str) -> () {
    xflow::util::fs::init_new_model_dir(path);
}

fn subcommand_validate(path: &str) -> () {
    let model = load_model(path);
    let path_sep = "/".to_owned();
    for xflow in &model.doc.xflows {
        let errors = xflow::validation::xflow::Validation::validate(&xflow);
        if errors.len() > 0 {
            for error in &errors {
                println!("XFlow '{}' : Error '{}' - Path '{}'",
                         xflow.id,
                         error.message,
                         error.paths.join(&path_sep));
            }
        } else {
            println!("XFlow '{}' validates OK", xflow.id);
        }
    }
}

fn subcommand_transform(input_format: &Format, output_format: &Format) -> () {
    let buffer = read_stdin();

    let model = match input_format {
        &Format::YAML => xflow::structure::model::ModelDocument::from_yaml(&buffer),
        &Format::JSON => xflow::structure::model::ModelDocument::from_json(&buffer),
    };

    match output_format {
        &Format::YAML => println!("{}", model.to_yaml()),
        &Format::JSON => println!("{}", model.to_json()),
    }
}

fn subcommand_build(path: &str, locale: &str) -> () {
    let model = load_model(path).as_locale(locale).unwrap();

    let mut pages_html = Vec::<String>::new();

    for page in &model.doc.pages {
        pages_html.push(xflow::generation::page_to_react_component::output_html(&page));
    }

    println!("{:?}", pages_html);
}

fn subcommand_import(path: &str, input_format: &Format) -> () {
    let buffer = read_stdin();

    let model = match input_format {
        &Format::YAML => xflow::structure::model::ModelDocument::from_yaml(&buffer),
        &Format::JSON => xflow::structure::model::ModelDocument::from_json(&buffer),
    };

    let _ = xflow::util::fs::model_to_fs(&model, &path).unwrap();

}

fn subcommand_export(path: &str, output_format: &Format) -> () {

    let model = xflow::util::fs::model_from_fs(&path).unwrap();

    match output_format {
        &Format::YAML => println!("{}", model.to_yaml()),
        &Format::JSON => println!("{}", model.to_json()),
    }

}
