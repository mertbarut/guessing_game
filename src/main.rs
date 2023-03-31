use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input a number between 1 and 10.");

    let mut guess = String::new();
    let random_number = (rand::random::<u32>() % 9) + 1;

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("You guessed: {}The number was: {}", guess, random_number);
    if str::parse::<u32>(&guess.trim()).unwrap_or(0) == random_number {
        println!("You won!");
        return;
    }
    println!("You lost.")
}
