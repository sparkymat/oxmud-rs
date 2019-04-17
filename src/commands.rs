use std::fs;

pub fn display_motd() {
    let motd = fs::read_to_string("data/motd.txt");
    match motd {
	Ok(motd) => {
            println!("{}", motd);
        },
        Err(err) => {
            println!("Error: {}", err);
        },
    }
}

pub fn display_help() {
    let help = fs::read_to_string("data/help.txt");
    match help {
        Ok(help) => {
            println!("{}", help);
        },
        Err(err) => {
            println!("Error {}", err);
        },
    }
}
