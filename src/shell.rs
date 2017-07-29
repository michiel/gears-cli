use rustyline::error::ReadlineError;
use rustyline::Editor;
use gears;

use gears::structure::model::ModelDocument;

use app::AppState;

#[allow(dead_code)]
mod command_grammar {
    include!(concat!(env!("OUT_DIR"), "/command_grammar.rs"));
}

static HISTORY_FILE: &'static str = "history.gears-shell";

// On unix platforms you can use ANSI escape sequences
#[cfg(unix)]
static PROMPT: &'static str = "\x1b[1;32m>>\x1b[0m ";

// Windows consoles typically don't support ANSI escape sequences out
// of the box
#[cfg(windows)]
static PROMPT: &'static str = ">> ";

struct ShellSession<'a> {
    appstate: &'a AppState,
    model: &'a mut ModelDocument,
}

impl<'a> ShellSession<'a> {
    pub fn run_line(&mut self, line: &str) -> Result<(), ()> {
        match command_grammar::expression(&line) {
            Ok(cmd) => {
                self.run_command(&cmd);
                Ok(())
            }
            Err(err) => {
                println!("<< Expected : {:?}", err.expected);
                Err(())
            }
        }
    }

    pub fn run_command(&mut self, cmd: &Command) -> Result<(), ()> {
        // println!("<< Running command");
        match *cmd {
            Command::Set(ref key, ref val) => {
                println!("<< Setting {} to {}", key, val);
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
                self.run_command_help();
            }
            Command::Sync => {
                println!("<< sync");
                self.run_command_sync();
            }
            Command::Dsl(ref dsl_cmd) => {
                println!("<< component_dsl_command");
                self.run_command_dsl(dsl_cmd);
            }
        }
        Ok(())
    }

    pub fn run_command_list(&self, component: &ModelComponent) -> () {
        //
        // XXX: Use Document::summary in later versions
        //
        match *component {
            ModelComponent::XFlow => {
                for doc in &self.model.doc.xflows {
                    println!("XFlow: ID {:?} - {:?}", doc.id, doc.name);
                }
            }
            ModelComponent::Page => {
                for doc in &self.model.doc.pages {
                    println!("Page: ID {:?} - {:?}", doc.id, doc.name);
                }
            }
            ModelComponent::Translation => {
                for doc in &self.model.doc.translations {
                    println!(
                        "Translation: ID {:?} - {:?} - {:?}",
                        doc.id,
                        doc.name,
                        doc.doc.locale
                    );
                }
            }
        }
    }

    pub fn run_command_generate(&mut self, component: &ModelComponent, name: &str) -> () {
        match *component {
            ModelComponent::XFlow => {
                let mut doc = gears::structure::xflow::XFlowDocument::default();
                doc.name = name.to_string();
                println!("XFlow: ID {:?} - {:?}", doc.id, doc.name);
                self.model.doc.xflows.push(doc);
            }
            ModelComponent::Page => {
                let mut doc = gears::structure::page::PageDocument::default();
                doc.name = name.to_string();
                println!("Page: ID {:?} - {:?}", doc.id, doc.name);
                self.model.doc.pages.push(doc);
            }
            ModelComponent::Translation => {
                let _ = self.model.add_locale(&name);
                let _ = self.model.pad_all_translations();
            }
        }
    }

    pub fn run_command_help(&self) -> () {
        println!("<< Help");
        println!("<< Use Ctrl-D or Ctrl-C to exit");
        println!("<< All changes are in-memory until a sync command is issued");
        println!("<< Read commands are help, list");
        println!("<< Write commands are generate, destroy, sync");
    }

    pub fn run_command_sync(&self) -> () {
        match gears::util::fs::model_to_fs(
            &self.model.as_locale(&self.appstate.locale).unwrap(),
            &self.appstate.path_in,
        ) {
            Ok(_) => {
                println!("<< sync OK");
            }
            Err(err) => {
                println!("<< sync ERROR : {:?}", err);
            }
        }
    }

    pub fn run_command_dsl(&mut self, dsl_cmd: &ComponentDslCommand) -> () {
        println!("<< RUN COMMAND DSL");
        use gears::structure::domain::*;

        match *dsl_cmd {
            ComponentDslCommand::XFlow(ref xflow_command) => {
                println!("Unimplemented!");
            }
            ComponentDslCommand::Domain(ref domain_command) => {
                info!("Domain command: {:?}", domain_command);
                match *domain_command {
                    DomainCommand::AddEntity(ref entity) => {
                        let entity = Entity {
                            name: entity.clone(),
                            attributes: Attributes::new(),
                            references: References::new(),
                        };
                        self.model.doc.domain.doc.entities.push(entity);
                    }
                    DomainCommand::RemoveEntity(ref entity) => {
                        let entities = self.model.doc.domain.doc.entities.clone();

                        self.model.doc.domain.doc.entities = entities
                            .into_iter()
                            .filter({
                                |e| e.name.ne(entity)
                            })
                            .collect();
                    }
                    DomainCommand::AlterEntity(ref entity, ref entity_b) => {
                        error!("NOT IMPLEMENTED");
                    }
                    DomainCommand::RenameEntity(ref entity, ref entity_b) => {
                        error!("NOT IMPLEMENTED");
                    }
                    DomainCommand::AddAttribute(ref entity, ref attribute, ref attribute_type) => {
                        let attribute = Attribute {
                            name: attribute.to_string(),
                            vtype: attribute_type.to_string(),
                            default: "".to_owned(),
                            validations: Vec::<Validation>::new(),
                        };
                        error!("NOT IMPLEMENTED");
                    }
                    DomainCommand::RemoveAttribute(ref entity, ref attribute) => {
                        error!("NOT IMPLEMENTED");
                    }
                    DomainCommand::AlterAttribute(ref entity, ref attribute, ref vals) => {
                        error!("NOT IMPLEMENTED");
                    }
                    DomainCommand::RenameAttribute(ref entity, ref attribute) => {
                        error!("NOT IMPLEMENTED");
                    }
                }
            }
        }
    }
}

pub fn shell(model: &mut ModelDocument, appstate: &AppState) -> () {
    println!("<< Running gears-shell");
    let mut shell_session = ShellSession {
        appstate: appstate,
        model: model,
    };

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if let Err(_) = rl.load_history(HISTORY_FILE) {
        println!("<< No previous history.");
    }

    loop {
        let readline = rl.readline(PROMPT);
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                // println!("Line: {}", line);
                shell_session.run_line(&line);
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
    rl.save_history(HISTORY_FILE).unwrap();
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
    Dsl(ComponentDslCommand),
}

#[derive(Debug)]
pub enum ComponentDslCommand {
    Domain(DomainCommand),
    XFlow(XFlowCommand),
}

#[derive(Debug)]
pub enum DomainCommand {
    AddEntity(String),
    RemoveEntity(String),
    AlterEntity(String, String),
    RenameEntity(String, String),
    AddAttribute(String, String, String),
    RemoveAttribute(String, String),
    AlterAttribute(String, String, String),
    RenameAttribute(String, String),
}

#[derive(Debug)]
pub enum XFlowCommand {
    AddNode(String),
}
