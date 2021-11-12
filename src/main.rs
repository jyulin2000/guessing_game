use std::io;

fn main() {
    println!("I'm thinking of a number. What is it?");

    println!("Input your guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
