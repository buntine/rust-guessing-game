extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("!!! GUESS THE RANDOM NUMBER (1 - 100) !!!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your number: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .ok()
        .expect("Please number a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Equal   => println!("You win!"),
        Ordering::Greater => println!("Too large!"),
    }
}
