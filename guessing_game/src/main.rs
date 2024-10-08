use rand::Rng;
use std::{cmp::Ordering, io};

// fn
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess...");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line :(");

        // let guess: u32 = guess.trim().parse().expect("Please enter a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small"),
            Ordering::Equal => {
                println!("Good guess, great job ! :) ");
                break;
            }
            Ordering::Greater => println!("To big"),
        }
    }
}
