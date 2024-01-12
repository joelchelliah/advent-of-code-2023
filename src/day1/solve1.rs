use std::io::{BufRead, BufReader};
use std::fs::File;

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
        for character in line.chars() {
            if character.is_digit(10) {
                if first_digit == 0 {
                    first_digit = character.to_digit(10).unwrap();
                }
                last_digit = character.to_digit(10).unwrap();
            }
        }
        let number = first_digit * 10 + last_digit;

        sum += number;
    }
    println!("Sum: {}", sum);
}
