use std::io::{BufRead, BufReader};
use std::fs::File;


fn rightmost_spelled_digit_to_digit(random_characters: &str) -> u32 {
    let valid_spelled_digits = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut number: u32 = 0;

    for (index, valid_spelled_digit) in valid_spelled_digits.iter().enumerate() {
        if random_characters.ends_with(valid_spelled_digit) {
            number = index as u32 + 1;
        }
    }
    number
}

pub fn solve() {
    let file = File::open("src/day1/calibration.txt").expect("💣");
    let reader = BufReader::new(file);


    let mut sum: u32 = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.trim().is_empty() {
            break;
        }

        let mut first_digit: u32 = 0;
        let mut last_digit: u32 = 0;
        let mut iterated_chars = String::new();

        for character in line.chars() {
            if character.is_digit(10) {
                let current_digit = character.to_digit(10).unwrap();
                if first_digit == 0 {
                    first_digit = current_digit;
                }
                last_digit = current_digit;
            } else {
                iterated_chars.push(character);
                let spelled_digit = rightmost_spelled_digit_to_digit(&iterated_chars);
                if spelled_digit == 0 {
                    continue;
                }

                if first_digit == 0 {
                    first_digit = spelled_digit;
                }
                last_digit = spelled_digit;
            }
        }
        let number = first_digit * 10 + last_digit;

        sum += number;
    }

    println!("Sum: {}", sum);
}
