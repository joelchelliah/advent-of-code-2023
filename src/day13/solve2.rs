use std::io::{BufRead, BufReader};
use std::fs::File;

type Pattern = Vec<Vec<char>>;

fn might_have_smudge(row_a: &Vec<char>, row_b: &Vec<char>) -> bool {
    row_a.iter().enumerate().filter(|(i, _)| row_b[*i] != row_a[*i]).count() == 1
}

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

        let mut smudge_fixed = false;

        for y in 1..pattern.len() {
            let mut has_reflection = true;

            for (i, yi) in (y..pattern.len()).enumerate() {
                let reflected_yi = (yi as isize)-(2 * i as isize)-1;
                if reflected_yi < 0 { break }

                let row = &pattern[yi];
                let reflected_row = &pattern[reflected_yi as usize];

                if row == reflected_row {
                    continue;
                } else if might_have_smudge(row, reflected_row) && !smudge_fixed {
                    smudge_fixed = true;
                    continue;
                } else {
                    has_reflection = false;
                    break;
                }
            }
            if has_reflection && smudge_fixed {
                count += y;
                processed_pattern_indices.push(pattern_index);
                break;
            } else {
                smudge_fixed = false;
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
