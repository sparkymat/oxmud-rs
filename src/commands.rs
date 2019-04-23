use std::fs;

pub fn display_motd() {
    match fs::read_to_string("data/motd.txt") {
	Ok(motd) => {
            println!("{}", motd);
        },
        Err(err) => {
            println!("Error: {}", err);
        },
    }
}

pub fn display_help() {
    match fs::read_to_string("data/help.txt") {
        Ok(help) => {
            println!("{}", help);
        },
        Err(err) => {
            println!("Error {}", err);
        },
    }
}
