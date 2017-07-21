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
                    println!("Translation: ID {:?} - {:?} - {:?}",
                             doc.id,
                             doc.name,
                             doc.doc.locale);
                }
            }
        }
    }

    pub fn run_command_generate(&mut self, component: &ModelComponent, name: &str) -> () {
        match *component {
            ModelComponent::XFlow => {
                let mut doc = gears::structure::xflow::XFlowDocument::default();
                doc.name = name.to_string();
                self.model.doc.xflows.push(doc);
            }
            ModelComponent::Page => {
                let mut doc = gears::structure::page::PageDocument::default();
                doc.name = name.to_string();
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
        match gears::util::fs::model_to_fs(&self.model.as_locale(&self.appstate.locale).unwrap(),
                                           &self.appstate.path_in) {
            Ok(_) => {
                println!("<< sync OK");
            }
            Err(err) => {
                println!("<< sync ERROR : {:?}", err);
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
}
