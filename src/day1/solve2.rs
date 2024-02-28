use std::collections::HashMap;

use crate::util::read_lines;


fn rightmost_spelled_digit_to_digit(random_characters: &str, spelled_digit_map: &HashMap<&str, u32>) -> u32 {
    for (spelled_digit, &value) in spelled_digit_map {
        if random_characters.ends_with(spelled_digit) {
            return value;
        }
    }
    0
}

fn update_digits(first_digit: &mut u32, last_digit: &mut u32, digit: u32) {
    if digit == 0 { return }
    if *first_digit == 0 {
        *first_digit = digit;
    }
    *last_digit = digit;
}

pub fn solve() {
    let mut sum: u32 = 0;
    let spelled_digit_map: HashMap<&str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ].iter().cloned().collect();

    for line in read_lines("src/day1/calibration.txt").unwrap() {
        let line = line.unwrap();
        let mut first_digit: u32 = 0;
        let mut last_digit: u32 = 0;
        let mut iterated_chars = String::new();

        for character in line.chars() {
            if character.is_digit(10) {
                let current_digit = character.to_digit(10).unwrap();

                update_digits(&mut first_digit, &mut last_digit, current_digit);
            } else {
                iterated_chars.push(character);
                let spelled_digit = rightmost_spelled_digit_to_digit(&iterated_chars, &spelled_digit_map);

                update_digits(&mut first_digit, &mut last_digit, spelled_digit);
            }
        }
        sum += first_digit * 10 + last_digit;
    }

    println!("Sum: {}", sum);
}
