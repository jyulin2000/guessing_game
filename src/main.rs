use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("I'm thinking of a number between 1 and 100. What is it?");
    
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    loop {
        println!("Input your guess: ");

        let mut guess: String = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small :("),
            Ordering::Greater => println!("Too Big..."),
            Ordering::Equal => {
                println!("You got it friend");
                break;
            }
        }
    }
}
