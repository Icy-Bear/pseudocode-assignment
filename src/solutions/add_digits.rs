use std::io;

pub fn run() {
    let mut input = String::new();

    println!("Enter a number:");
    io::stdin().read_line(&mut input).unwrap();

    let mut number: u32 = input.trim().parse().unwrap();
    let original_number = number;
    let mut sum = 0;

    while number > 0 {
        let digit = number % 10;
        sum += digit;
        number /= 10;
    }

    println!("Sum of digits of {} is {}", original_number, sum);
}