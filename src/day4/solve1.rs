use std::io::{BufRead, BufReader};
use std::fs::File;

fn get_numbers(input: &str) -> Vec<&str> {
    input.trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>()
}

pub fn solve() {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    let file = File::open("src/day4/scratch_cards.txt").expect("💣");
    let reader = BufReader::new(file);

    let mut total_points = 0;

    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    for line in reader.lines() {
        let line = line.expect("Nuuuu! 💣");
        if line.trim().is_empty() {
            break;
        }

        let number_lists: Vec<&str> = line.split(":").collect::<Vec<&str>>()[1].split("|").collect();
        // [41, 48, 83, 86, 17]
        let winning_numbers: Vec<&str> = get_numbers(number_lists[0]);
        // [83, 86, 6, 31, 17, 9, 48, 53]
        let playing_numbers: Vec<&str> = get_numbers(number_lists[1]);

        let mut points = 0;

        for number in playing_numbers {
            if winning_numbers.contains(&number) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        total_points += points;
    }

    println!("Sum: {}", total_points);
}
