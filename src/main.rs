use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to guessing game!");

    println!("I generate a number between 1 and 100 (included).");
    
    let generated_number = rand::thread_rng().gen_range(1..=100);

    println!("Generated: {generated_number}");

    
    
    
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&generated_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Equal => { println!("You guessed right."); break;},
            Ordering::Greater => println!("Too big.")
        }
    }
}
