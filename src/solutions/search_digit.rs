use std::io;

pub fn run() {
    let mut number_input = String::new();
    let mut digit_input = String::new();

    println!("Enter a number:");
    io::stdin().read_line(&mut number_input).unwrap();

    println!("Enter the digit to search:");
    io::stdin().read_line(&mut digit_input).unwrap();

    let mut number: u32 = number_input.trim().parse().unwrap();
    let original_number = number;
    let target_digit: u32 = digit_input.trim().parse().unwrap();

    let mut found = false;

    while number > 0 {
        let digit = number % 10;

        if digit == target_digit {
            found = true;
            break;
        }

        number /= 10;
    }

    if found {
        println!("Digit {} is present in {}", target_digit, original_number);
    } else {
        println!("Digit {} is not present in {}", target_digit, original_number);
    }
}