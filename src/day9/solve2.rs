use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn solve() {
    // 0 3 6 9 12 15
    // 1 3 6 10 15 21
    // 10 13 16 21 30 45
    let file = File::open("src/day9/report.txt").expect("💣");
    let reader = BufReader::new(file);

    let mut histories: Vec<Vec<i32>> = Vec::new();

    // 0 3 6 9 12 15
    for line in reader.lines() {
        let line = line.expect("Nuuuu! 💣");
        if line.trim().is_empty() {
            break;
        }

        // [[0, 3, 6, 9, 12, 15], ...]
        histories.push(line.split(" ").collect::<Vec<&str>>().iter()
                           .filter(|digits: &&&str| !digits.is_empty())
                           .map(|digits| digits.parse::<i32>().unwrap())
                           .collect());
    }

    // [[[0, 3, 6, 9, 12, 15],
    //     [3, 3, 3, 3, 3],
    //      [0, 0, 0, 0]],
    // [[...], ...], ...]
    let sequence_of_diffs_list: Vec<Vec<Vec<i32>>> = histories.iter().map(|history| {
        let mut history = history.clone();
        let mut sequence_of_diffs: Vec<Vec<i32>> = vec![history.clone()];

        'outer: loop {
            let mut diffs: Vec<i32> = Vec::new();
            for i in 1..history.len() {
                diffs.push(history[i] - history[i - 1]);
            }
            sequence_of_diffs.push(diffs.clone());
            history = diffs.clone();

            if diffs.iter().all(|&diff| diff == 0) {
                break 'outer;
            }
        }
        sequence_of_diffs
    }).collect();

    let mut sum_prediction = 0;

    for sequence_of_diffs in sequence_of_diffs_list {
        let mut prediction = 0;

        for i in (0..sequence_of_diffs.len()).rev() {
            let diffs = sequence_of_diffs[i].clone();

            if diffs.iter().all(|&diff| diff == 0) {
                continue;
            } else {
                prediction = diffs[0] - prediction;
            }
        }
        sum_prediction += prediction;
    }

    println!("Answer: {}", sum_prediction);
}
