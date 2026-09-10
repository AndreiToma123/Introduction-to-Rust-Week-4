use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};

fn write_in_file(filename: &String, content: &String) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())
}

fn read_file(filename: & String) {
    let content = match fs::read_to_string(filename) {
        Ok(text) => text,
        Err(_) => {
            println!("Couldn't read file");
            return;
        }
    };
    println!("The contents of the file:\n\n{}", content);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No arguments were given.");
    } else if args.len() < 3 || args.len() > 4 {
        println!("Unexpected number of arguments! Use as follows:");
        println!(
            "cargo run <filename> <operation> OR cargo run <filename> <operation> <from_filename>"
        );
        return;
    } else if args[2].to_lowercase() == "read" {
        if args.len() > 3 {
            println!("Only give three arguments when writing.");
        } else if args.len() == 3 {
            read_file(&args[1]);
        }
    } else if args[2].to_lowercase() == "write" {
        {
            if args.len() < 4 {
                println!("More arguments needed if not reading.");
            } else if args.len() == 4 {
                write_in_file(&args[1], &args[3]);
            }
        }
    }
}
