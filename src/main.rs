extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut editor = Editor::<()>::new();
    if editor.load_history("history.txt").is_err() {
        println!("No history");
    }

    loop {
        let line = editor.readline("> ");
        match line {
            Ok(line) => {
                editor.add_history_entry(line.as_ref());
                if line.trim() == "quit" {
                    break;
                }
                println!("Thank you for {}", line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("Enter 'quit' to exit")
            }
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
