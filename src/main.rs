mod commands;
mod setup;
extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    commands::display_motd();

    setup::load_database();

    let mut editor = Editor::<()>::new();
    if editor.load_history("history.txt").is_err() {
        println!("No history");
    }

    loop {
        match editor.readline("> ") {
            Ok(line) => {
                editor.add_history_entry(line.as_ref());
                if line.trim() == "quit" {
                    break;
                }

                match line.trim() {
                    "quit" => {
                        break;
                    }
                    "help" => {
                        commands::display_help();
                    }
                    "motd" => {
                        commands::display_motd();
                    }
                    _ => println!("Error: unrecognized command"),
                }
            }
            Err(ReadlineError::Interrupted) => println!("Enter 'quit' to exit"),
            Err(ReadlineError::Eof) => {
                println!("Bye bye");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
    editor.save_history("history.txt").unwrap();
}
