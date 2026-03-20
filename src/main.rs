mod solutions;

use std::io;

fn main() {
    println!("Pseudocode Assignment");
    println!("1. Add Digits of a Number");
    println!("2. Search a Digit in a Number");
    println!("Enter your choice:");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => solutions::add_digits::run(),
        "2" => solutions::search_digit::run(),
        _ => println!("Invalid choice"),
    }
}