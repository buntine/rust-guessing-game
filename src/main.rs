use std::io;

fn main() {
    println!("!!! GUESS THE RANDOM NUMBER (1 - 100) !!!");

    println!("Please input yuor number: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
