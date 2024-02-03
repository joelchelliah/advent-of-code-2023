use std::io::{BufRead, BufReader};
use std::fs::File;

fn count_arrangements(springs: &str, group_sizes: &Vec<u32>) -> u32 {
    let is_broken_or_wildcard = |c: char| c == '#' || c == '?';
    let remaining_broken_or_wildcard = springs.chars().filter(|c| is_broken_or_wildcard(*c)).count() as u32;

    if group_sizes.is_empty() {
        // Cannot have any broken springs remaining!
        if springs.chars().any(|c| c == '#') {
            0
        } else {
            1
        }
    // Not possible to use up all group values!
    } else if remaining_broken_or_wildcard < group_sizes.iter().sum::<u32>() {
        0
    } else if springs.starts_with(".") {
        let next_broken_or_wildcard_index = springs.chars().position(is_broken_or_wildcard).unwrap();
        let new_springs = springs[next_broken_or_wildcard_index..].to_string();

        count_arrangements(&new_springs, group_sizes)
    } else {
        let group_size = group_sizes[0];
        let candidate = springs[0..(group_size as usize)].to_string();

        let count_using_candidate = if candidate.chars().all(is_broken_or_wildcard) {
            if (group_size as usize) == springs.len() {
                1
            } else if (group_size as usize) > springs.len() {
                0
            } else {
                let new_springs = springs[(group_size as usize)..].to_string();
                let new_group_sizes = group_sizes[1..].to_vec();

                // Cannot skip a broken spring!
                if new_springs.starts_with('#') {
                    0
                // +1 if exactly 1 wildcard left and no more groups of unplaced springs.
                } else if new_springs.starts_with('?') && new_springs.len() == 1 && new_group_sizes.is_empty() {
                    1
                } else {
                    count_arrangements(&new_springs[1..].to_string(), &new_group_sizes)
                }
            }
        } else {
            0
        };
        // Cannot skip a broken spring!
        let count_when_skipping_one_step = if springs.starts_with('#') {
            0
        } else {
            count_arrangements(&springs[1..].to_string(), group_sizes)
        };

        count_using_candidate + count_when_skipping_one_step
    }

}

pub fn solve() {
    // ???.### 1,1,3
    // .??..??...?##. 1,1,3
    // ?#?#?#?#?#?#?#? 1,3,1,6
    let file = File::open("src/day12/condition_records.txt").expect("💣");
    let reader = BufReader::new(file);

    let mut sum_arrangements = 0;

    // ???.### 1,1,3
    for line in reader.lines() {
        let line = line.expect("Nuuuu! 💣");
        if line.trim().is_empty() {
            break;
        }

        let info_parts: Vec<&str> = line.split(" ").collect();
        let springs = info_parts[0].trim();
        let group_sizes: Vec<u32> = info_parts[1].trim().split(",").map(|size| size.trim().parse::<u32>().unwrap()).collect();

        sum_arrangements += count_arrangements(springs, &group_sizes);
    }

    println!("Sum: {}", sum_arrangements);
}
