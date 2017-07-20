use rustyline::error::ReadlineError;
use rustyline::Editor;
use gears;

use gears::structure::model::ModelDocument;

use app::AppState;

#[allow(dead_code)]
mod command_grammar {
    include!(concat!(env!("OUT_DIR"), "/command_grammar.rs"));
}

struct ShellSession<'a> {
    appstate: &'a AppState,
    model: &'a mut ModelDocument,
}

impl<'a> ShellSession<'a> {
    pub fn run_command(&mut self, cmd: &Command) -> Result<(), ()> {
        println!("Running command");
        match *cmd {
            Command::Set(ref key, ref val) => {
                println!("Setting {} to {}", key, val);
            }
            Command::List(ref component) => {
                debug!("Listing {:?}", component);
                self.run_command_list(&component);
            }
            Command::Generate(ref component, ref val) => {
                debug!("Generating {:?} with name {}", component, val);
                self.run_command_generate(&component, &val);
            }
            Command::Destroy(ref component, ref val) => {
                println!("Destroying {:?} with name {}", component, val);
            }
            Command::Help => {
                println!("Help");
                self.run_command_help();
            }
            Command::Sync => {
                println!("Sync");
                self.run_command_sync();
            }
        }
        Ok(())
    }

    pub fn run_command_list(&self, component: &ModelComponent) -> () {
        match *component {
            ModelComponent::XFlow => {
                for doc in &self.model.doc.xflows {
                    println!("XFlow: ID {:?}", doc.id);
                }
            }
            ModelComponent::Page => {
                for doc in &self.model.doc.pages {
                    println!("Page: ID {:?}", doc.id);
                }
            }
            ModelComponent::Translation => {
                for doc in &self.model.doc.translations {
                    println!("Translation: ID {:?}", doc.id);
                }
            }
        }
    }

    pub fn run_command_generate(&mut self, component: &ModelComponent, name: &str) -> () {
        match *component {
            ModelComponent::XFlow => {
                let mut doc = gears::structure::xflow::XFlowDocument::default();
                self.model.doc.xflows.push(doc);
            }
            ModelComponent::Page => {
                let mut doc = gears::structure::page::PageDocument::default();
                self.model.doc.pages.push(doc);
            }
            ModelComponent::Translation => {}
        }
    }

    pub fn run_command_help(&self) -> () {
        println!("Help command");
    }

    pub fn run_command_sync(&self) -> () {
        let _ = gears::util::fs::model_to_fs(&self.model.as_locale(&self.appstate.locale).unwrap(),
                                             &self.appstate.path_in)
            .unwrap();
    }
}

pub fn shell(model: &mut ModelDocument, appstate: &AppState) -> () {
    println!("Running gears-shell");
    let mut shell_session = ShellSession {
        appstate: appstate,
        model: model,
    };

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if let Err(_) = rl.load_history("history.gears-shell") {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                println!("Line: {}", line);
                match command_grammar::expression(&line) {
                    Ok(cmd) => {
                        shell_session.run_command(&cmd);
                    }
                    Err(err) => {
                        println!("Expected : {:?}", err.expected);
                    }
                };
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}

#[derive(Debug)]
pub enum ModelComponent {
    XFlow,
    Page,
    Translation,
}

#[derive(Debug)]
pub enum Command {
    Help,
    Sync,
    Set(String, String),
    List(ModelComponent),
    Generate(ModelComponent, String),
    Destroy(ModelComponent, String),
}
