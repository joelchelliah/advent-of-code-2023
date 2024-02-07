use std::io::{BufRead, BufReader};
use std::fs::File;

fn run_hash_algorithm(input: String, current_value: u64) -> u64 {
    if input.is_empty() { return current_value };

    let mut chars = input.chars();
    let first = chars.next().unwrap();
    let current_value = (current_value + (first as u8 as u64)) * 17 % 256;

    run_hash_algorithm(chars.collect::<String>(), current_value)
}

pub fn solve() {
    // rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    let file = File::open("src/day15/initialization_sequence.txt").expect("💣");
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap().unwrap().to_owned();

    let steps = line.split(",").collect::<Vec<&str>>();

    let sum = steps.iter().fold(0, |acc, step| {
        acc + run_hash_algorithm(step.to_string(), 0)
    });

    println!("Sum: {}", sum);
}
