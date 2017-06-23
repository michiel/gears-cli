extern crate xflow;
#[macro_use]
extern crate clap;
use clap::App;
use std::io::{self, Read};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let config = matches.value_of("config").unwrap_or("project.conf");

    if let Some(matches) = matches.subcommand_matches("new") {
        if matches.is_present("debug") {
            println!("new : Printing debug info...");
        } else {
            println!("new : Printing normally...");
        }
    }

    if let Some(matches) = matches.subcommand_matches("export-json") {
        let model = xflow::util::fs::model_from_fs(&"/home/michiel/dev/github/xflow-rust/resource/projects/basic")
            .unwrap();
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


}
