use std::io;
use std::cmp::Ordering;
use rand::*;

fn main() {
    print!("Guess what the secret number is ");
    print!("\n");
    
    let secret: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error reading line from input");
        let guess: i32 = guess.trim().parse().expect("Please type a number!");
        println!("Your guess: {guess}");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal =>  {
                println!("Perfect!");
                break;
            }
        }
    }
}
