use std::io::{BufRead, BufReader};
use std::fs::File;

type Pattern = Vec<Vec<char>>;

fn process_patterns(patterns: &[Vec<Vec<char>>], processed_pattern_indices: &mut Vec<usize>) -> usize {
    let transpose = !processed_pattern_indices.is_empty();
    let mut pattern_index = 0;
    let mut count = 0;

    for pattern in patterns {
        if processed_pattern_indices.contains(&pattern_index) {
            pattern_index += 1;
            continue;
        }
        let pattern = if transpose {
            (0..pattern[0].len()).map(|i| pattern.iter().map(|row| row[i].clone()).collect()).collect()
        } else {
            pattern.clone()
        };

        for y in 1..pattern.len() {
            if (y..pattern.len()).enumerate().all(|(i, yi)| {
                let reflected_yi = (yi as isize)-(2 * i as isize)-1;

                reflected_yi < 0 || pattern[yi] == pattern[reflected_yi as usize]
            }) {
                count += y;
                processed_pattern_indices.push(pattern_index);
                break;
            }
        }
        pattern_index += 1;
    }

    count
}

pub fn solve() {
    // #.##..##.
    // ..#.##.#.
    // ##......#
    let file = File::open("src/day13/patterns.txt").expect("💣");
    let reader = BufReader::new(file);

    let mut patterns: Vec<Pattern> = Vec::new();
    let mut pattern: Pattern = Vec::new();

    // #.##..##.
    for line in reader.lines() {
        let line = line.expect("Nuuuu! 💣");

        if line.trim().is_empty() && pattern.len() > 0 {
            patterns.push(pattern);
            pattern = Vec::new();
            continue;
        }
        pattern.push(line.chars().collect());
    }
    if pattern.len() > 0 { patterns.push(pattern) }

    let mut processed_pattern_indices: Vec<usize> = Vec::new();

    let upper_row_count = process_patterns(&patterns, &mut processed_pattern_indices);
    let left_col_count = process_patterns(&patterns, &mut processed_pattern_indices);

    println!("Sum: {}", (left_col_count + 100 * upper_row_count) as i64);
}
