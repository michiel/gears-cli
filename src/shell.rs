use rustyline::error::ReadlineError;
use rustyline::Editor;

#[allow(dead_code)]
mod command_grammar {
    include!(concat!(env!("OUT_DIR"), "/command_grammar.rs"));
}

pub fn shell() -> () {
    println!("Running gears-shell");
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
                        run_command(&cmd);
                    }
                    Err(err) => {
                        println!("Error : {:?}", err);
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

pub enum ModelComponent {
    XFlow,
    Page,
    Translation,
}

pub enum Command {
    Help,
    Set(String, String),
    Generate(ModelComponent, String),
    Destroy(ModelComponent, String),
}

fn run_command(cmd: &Command) -> Result<(), ()> {
    println!("Running command");
    Ok(())
}
