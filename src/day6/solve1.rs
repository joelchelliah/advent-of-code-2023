use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn solve() {
    // Time:      7  15   30
    // Distance:  9  40  200
    let file = File::open("src/day6/record.txt").expect("💣");
    let reader = BufReader::new(file);

    let mut times: Vec<i32> = Vec::new();
    let mut distances: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Nuuuu! 💣");
        if line.trim().is_empty() {
            break;
        }
        let line_parts: Vec<&str> = line.split(":").collect::<Vec<&str>>();

        if line_parts[0].starts_with("Time") {
            // [7, 15, 30]
            times = line_parts[1].trim().split(" ").collect::<Vec<&str>>()
                .iter()
                .filter(|time| !time.is_empty())
                .map(|time| time.parse::<i32>().unwrap())
                .collect();
        } else {
            // [9, 40, 200]
            distances = line_parts[1].trim().split(" ").collect::<Vec<&str>>()
                .iter()
                .filter(|distance| !distance.is_empty())
                .map(|distance| distance.parse::<i32>().unwrap())
                .collect();
        }
    }

    let mut total_margin_of_error = 1;

    for i in 0..times.len() {
        let max_time = times[i];
        let record_distance = distances[i];
        let mut num_record_breaks = 0;

        for hold_time in 1..max_time {
            let run_time = max_time - hold_time;
            let distance = hold_time * run_time;

            if distance > record_distance {
                num_record_breaks += 1;
            }
        }

        total_margin_of_error *= num_record_breaks;
    }

    println!("Answer: {}", total_margin_of_error);
}
