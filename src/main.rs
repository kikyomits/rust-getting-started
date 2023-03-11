use ferris_says::say;
use rocket::http::SameSite::Strict;
use std::io::stdin;

fn main() {
    println!("Please input your guess.");
    let mut guess = String::new();
    stdin().read_line(&mut guess).expect("TODO: panic message");
    println!("You guessed: {guess}");
}
