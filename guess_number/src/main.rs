use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game!");

    let random_number = rand::thread_rng().gen_range(1..=100);
    let mut is_guessed = false;

    while !is_guessed {
        let guess = get_user_guess();

        if guess < random_number {
            println!("Too small!");
        } else if guess > random_number {
            println!("Too big!");
        } else {
            is_guessed = true;
        }
    }

    println!("You win!");
}

fn get_user_guess() -> u32 {
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        match guess.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}
