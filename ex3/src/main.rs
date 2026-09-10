use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::process::exit;

fn start_duel() {
    println!("FIRE!!!");
    thread::sleep(Duration::from_secs(5));
    thread::spawn(|| {
        opponent();
    });

    io::stdout().flush().unwrap();

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line.");

    match user_input.trim() {
        "f" => {
            println!("You fire first!");
            exit(0);
        },
        _ => {
            println!("Oh no! You missed!\nOpponent shoots first!");
        },
    }
}

fn opponent() {
    thread::sleep(Duration::from_secs(3));
    println!("Opponent shoots first!");
    std::process::exit(0);
}

fn main() {
    start_duel();
}
