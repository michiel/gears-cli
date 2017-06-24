extern crate xflow;
#[macro_use]
extern crate clap;
use clap::App;
use std::io::{self, Read};

fn load_model(path: &str) -> xflow::structure::model::ModelDocument {
    let model = xflow::util::fs::model_from_fs(path).unwrap();
    model
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let config = matches.value_of("config").unwrap_or("project.conf");
    let path = matches.value_of("path").unwrap_or(".");

    if let Some(matches) = matches.subcommand_matches("new") {
        if matches.is_present("debug") {
            println!("new : Printing debug info...");
        } else {
            println!("new : Printing normally...");
        }
    }

    if let Some(matches) = matches.subcommand_matches("export-json") {
        let model = xflow::util::fs::model_from_fs(&path).unwrap();
        println!("{}", model.to_json());
    }

    if let Some(matches) = matches.subcommand_matches("import-json") {

        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        handle.read_to_string(&mut buffer).unwrap();
        let model = xflow::structure::model::ModelDocument::from_json(&buffer);

        println!("{}", model.to_json());
    }

    if let Some(matches) = matches.subcommand_matches("validate") {

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

    if let Some(matches) = matches.subcommand_matches("generate") {
        let model = load_model(path);

        let mut forms_html = Vec::<String>::new();

        for form in &model.doc.forms {
            forms_html.push(xflow::generation::vue_form::output_html(&form));
        }

        println!("{:?}", forms_html);
    }


}
