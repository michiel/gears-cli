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
    pub fn run_line(&mut self, line: &str) -> Result<(), String> {
        match command_grammar::expression(&line) {
            Ok(cmd) => {
                debug!("run_line parsed to : {:?}", cmd);
                self.run_command(&cmd)
            }
            Err(err) => {
                println!("<< Parsing error (shell) : {:?}", err);
                let msg = format!("{:?}", err);
                Err(msg)
            }
        }
    }

    pub fn run_command(&mut self, cmd: &Command) -> Result<(), String> {
        // println!("<< Running command");
        match *cmd {
            Command::Nop => Ok(()),
            Command::Set(ref key, ref val) => {
                println!("<< Setting {} to {}", key, val);
                Ok(())
            }
            Command::Help => {
                self.run_command_help();
                Ok(())
            }
            Command::Sync => {
                println!("<< sync");
                self.run_command_sync()
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

    pub fn run_command_sync(&self) -> Result<(), String> {
        match gears::util::fs::model_to_fs(
            &self.model.as_locale(&self.appstate.locale).unwrap(),
            &self.appstate.path_in,
        ) {
            Ok(_) => {
                println!("<< sync OK");
                Ok(())
            }
            Err(err) => {
                println!("<< sync ERROR : {:?}", err);
                Err(format!("{:?}", err))
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
                debug!("readline input {:?}", line);
                rl.add_history_entry(&line);
                match shell_session.run_line(&line) {
                    Ok(_) => {}
                    Err(err) => {
                        println!("<< readline error '{:?}' - Error : {:?}", line, err);
                        error!("readline, runline {:?}", err);
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
    rl.save_history(HISTORY_FILE).unwrap();
}

#[derive(Debug)]
pub enum Command {
    Nop,
    Help,
    Sync,
    Set(String, String),
}
