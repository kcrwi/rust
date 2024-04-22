use std::io;
use std::cmp::Ordering;
use rand::*;

fn main() {
    print!("Guess what the secret number is ");
    print!("\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

                
    loop {}
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too large"),
        Ordering::Equal => println!("You win!"),
    }

    
}
