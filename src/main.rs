use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input a number between 1 and 10. You have 3 attempts.");

    let mut guess = String::new();
    let random_number = rand::thread_rng().gen_range(1, 10);
    let mut attempts = 0;

    loop {
        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        attempts = attempts + 1;
        let remaining_attempts = 3 - attempts;
        println!("You guessed: {}", guess);
        let parsed_guess = str::parse::<u32>(&guess.trim())
            .unwrap_or(0)
            .cmp(&random_number);
        match parsed_guess {
            Ordering::Greater => {
                println!("Too big! Remaining attempts: {}", remaining_attempts);
                if remaining_attempts == 0 {
                    println!("{}", "You lose.".red());
                    break;
                }
                continue;
            }
            Ordering::Less => {
                println!("Too small! Remaining attempts: {}", remaining_attempts);
                if remaining_attempts == 0 {
                    println!("{}", "You lose.".red());
                    break;
                }
                continue;
            }
            Ordering::Equal => {
                println!("{}", "You win!".green());
                println!("Number of attempts took you to win: {}", attempts);
                break;
            }
        }
    }
}
