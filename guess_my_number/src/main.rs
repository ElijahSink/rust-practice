use rand::prelude::*;
use std::io::*;

static RANGE: (u32, u32) = (1, 100);

fn main() {
    println!("Welcome to Guess My Number!");
    println!(
        "I'll pick a number between {} and {} (inclusive)",
        RANGE.0, RANGE.1
    );
    println!("And you'll try to guess it.");
    loop {
        let secret_number = thread_rng().gen_range(RANGE.0..=RANGE.1);
        println!("Okay, I've got my secret number.");
        let mut guess: u32 = 0;
        let mut guesses: u32 = 0;

        while guess != secret_number {
            guess = get_guess();
            guesses += 1;
            if guess == secret_number {
                println!("Nice! You got it in {} guesses", guesses);
            } else if guess > secret_number {
                println!("Too high!");
            } else if guess < secret_number {
                println!("Too low!");
            }
        }

        if !input_bool("Would you like to play again?", true) {
            break;
        }
    }
}

fn get_guess() -> u32 {
    println!("Enter your guess: ");

    let mut input = String::new();
    loop {
        stdin().read_line(&mut input).unwrap_or_default();

        match input.trim().parse::<u32>() {
            Ok(guess) if RANGE.0 <= guess && guess <= RANGE.1 => return guess,
            _ => println!(
                "That's not a valid guess. Please enter a number between {} and {}",
                RANGE.0, RANGE.1
            ),
        }
    }
}

/// Prompts the user with
/// `<prompt> (Y/n): ` or `<prompt> (y/N): `
/// depending on the value of `default`
fn input_bool(prompt: &str, default: bool) -> bool {
    let yes = if default { "Y" } else { "y" };
    let no = if default { "n" } else { "N" };

    println!("{prompt} ({yes}/{no}): ");

    let mut input = String::new();
    loop {
        stdin().read_line(&mut input).unwrap_or_default();
        match input.trim().to_lowercase().as_str() {
            "" => return default,
            "y" => return true,
            _ => return false,
        }
    }
}
