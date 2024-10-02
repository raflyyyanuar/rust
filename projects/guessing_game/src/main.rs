use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("\n=== GUESS THE NUMBER ===");

    let max_limit: u32 = 100;
    let mut guesses = (max_limit as f32).log2().ceil() as i32;

    println!("The secret number is between 1 and {max_limit}");
    println!("You can only guess {guesses} times!!");

    let secret_number = rand::thread_rng().gen_range(1..=max_limit);
    
    loop {
        match guesses.cmp(&0) {
            Ordering::Equal => {
                println!("\nSorry, you ran out of guesses!");
                break;
            },
            _ => println!("\nGuesses remaining {guesses}"),
        };

        print!("Please input your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        guesses -= 1;
    }

    println!("\nThank you for playing my guessing game!");
}