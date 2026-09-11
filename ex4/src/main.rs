use std::io;
use std::process::exit;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn create_threads(number: i64) {
    let (tx, rx) = mpsc::channel();
    let (stop_tx, stop_rx) = mpsc::channel::<()>();
    let mut remaining_balance = number;

    let catch_thieves = tx.clone();
    thread::spawn(move || loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Could not read line.");
        if user_input.trim() == "catch" {
            catch_thieves.send(0).unwrap();
            break;
        }
    });

    let normal_thief = tx.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));
        println!("ALERT!!! Someone stole $10,000 from you!");
        normal_thief.send(10000).unwrap();
    });

    let greedy_thief = tx.clone();
    thread::spawn(move || loop {
        match stop_rx.recv_timeout(Duration::from_secs(3)) {
            Ok(_) => break,
            Err(mpsc::RecvTimeoutError::Timeout) => {
                println!("ALERT!!! Someone stole $35,000 from you!");
                greedy_thief.send(35000).unwrap();
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    });

    drop(tx);

    loop {
        match rx.recv() {
            Ok(value) => {
                if value == 0 {
                    println!("The thieves have left.");
                    exit(0);
                }
                if remaining_balance < value {
                    println!("You lost all your money!");
                    exit(0);
                } else {
                    remaining_balance -= value;
                    println!("Funds left: {}", remaining_balance);
                }

                if number == 1000000 && remaining_balance < 600000 {
                    let _ = stop_tx.send(());
                }
            }
            Err(_) => break,
        }
    }
}

fn main() {
    println!("Do you have a million dollars? | y = yes, n = no");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Could not read line.");

    let current_balance: i64 = match user_input.trim() {
        "y" => {
            println!("All right then, millionaire.");
            1000000
        }
        "n" => {
            println!("Let's just assume you have $100,000 then.");
            100000
        }
        _ => {
            println!("Wrong input. Try again");
            return;
        }
    };
    create_threads(current_balance);
}
