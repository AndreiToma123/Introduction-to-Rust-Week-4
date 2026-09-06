use std::io;
use std::fs;
use std::process::exit;

fn read_file() {
    let content = match fs::read_to_string("read.txt") {
        Ok(text) => text,
        Err(_) => {
            println!("Couldn't read file");
            return;
        }
    };
    println!("{}", content);
}

fn prank_user() {
    println!("$ You have received an email.");
}

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let input = input.trim();

        if input.to_lowercase() == "read" {
            println!();
            read_file();
        }
        else if input.to_lowercase() == "prank" {
            println!();
            prank_user();
        }
        else if input.to_lowercase() == "help" {
            print!("$ Commands: read, prank, help, end.");
        }
        else if input.to_lowercase() == "end" {
            // exit(0);
            break;
        }
        else {
            println!("Invalid command. Try again.");
        }
    }

}
