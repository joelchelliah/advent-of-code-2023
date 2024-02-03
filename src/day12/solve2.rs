use std::io::{BufRead, BufReader};
use std::fs::File;
use std::iter::repeat;
use std::collections::HashMap;

fn count_arrangements(springs: &str, group_sizes: &[u32],  broken_springs_length: u32, cache: &mut HashMap<String, u64>) -> u64 {
    let key = format!("{}-{}-{}", springs, group_sizes.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(","), broken_springs_length);

    if cache.contains_key(key.as_str()) {
        return *cache.get(key.as_str()).unwrap();
    }

    if group_sizes.is_empty() {
        return if springs.contains('#') { 0 } else { 1 };
    }

    if group_sizes[0] == broken_springs_length {
        if springs.starts_with("#") {
            0
        } else if springs.is_empty() && group_sizes.len() == 1 {
            1
        } else if !springs.is_empty() {
            let count = count_arrangements(&springs[1..], &group_sizes[1..], 0, cache);

            cache.insert(key, count);
            count
        } else {
            0
        }
    } else if springs.is_empty() {
        0
    } else if springs.starts_with(".") {
        let count = if broken_springs_length == 0 {
            count_arrangements(&springs[1..], group_sizes, broken_springs_length, cache)
        } else {
            0
        };
        cache.insert(key, count);
        count
    } else if springs.starts_with("#") {
        let count = count_arrangements(&springs[1..], group_sizes, broken_springs_length + 1, cache);
        cache.insert(key, count);
        count
    } else {
        let count_if_placing_spring = count_arrangements(&springs[1..], group_sizes, broken_springs_length + 1, cache);
        let count_if_skipping_spring = if broken_springs_length == 0 {
            count_arrangements(&springs[1..], group_sizes, broken_springs_length, cache)
        } else {
            0
        };
        let count = count_if_placing_spring + count_if_skipping_spring;

        cache.insert(key, count);
        count
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
        let springs_orig = info_parts[0].trim();
        let group_sizes_orig: Vec<u32> = info_parts[1].trim().split(",").map(|size| size.trim().parse::<u32>().unwrap()).collect();

        let springs = repeat(springs_orig).take(5).collect::<Vec<_>>().join("?");
        let group_sizes = repeat(group_sizes_orig).take(5).flatten().collect::<Vec<_>>();
        let mut cache = HashMap::new();

        let num_arrangements = count_arrangements(&springs, &group_sizes, 0, &mut cache);

        sum_arrangements += num_arrangements;
    }

    println!("Sum: {}", sum_arrangements);
}
