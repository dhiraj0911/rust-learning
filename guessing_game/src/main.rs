use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;
fn main() {
    println!("Guess the numner!");

    println!("Please input your guess.");
    let secrete_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        println!("The secret number is: {}", secrete_number);
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secrete_number) {
            Ordering::Less => println!("{}","Too small".red()),
            Ordering::Equal => {
                println!("{}", "You guessed it!!".green());
                break;
            },
            Ordering::Greater => println!("{}","Too big".red()),
        }
    }
}
