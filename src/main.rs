use rand::Rng;
use std::cmp::Ordering;
use std::io;

// A simple guessing game that allows a user to guess upto 5 times, a secret number

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let max_number_of_guesses_allowed: i32 = 5;
    let mut number_of_guesses: i32 = 1;

    println!(
        "Guess the number! You have {} guesses with hints",
        max_number_of_guesses_allowed
    );

    while number_of_guesses <= max_number_of_guesses_allowed {
        let mut guess = String::new();
        println!("Let's Go > Guess number {}", number_of_guesses);
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read your guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Please enter a whole number");
                continue;
            }
        };

        let response = if number_of_guesses == 4 {
            "Last attempt"
        } else if number_of_guesses == 5 {
            ""
        } else {
            "Try again"
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed {}! Well, too small! {}", guess, response),
            Ordering::Greater => println!("You guessed {}! Well, too big! {}", guess, response),
            Ordering::Equal => {
                println!("You guessed {}! Coorect, You win!!!", guess);
                break;
            }
        }

        number_of_guesses += 1;
    }
    println!(
        "You lost {} times! Secret number is {}",
        number_of_guesses - 1,
        secret_number
    );
}
