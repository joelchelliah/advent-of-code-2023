use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

fn run_hash_algorithm(input: String, current_value: usize) -> usize {
    if input.is_empty() { return current_value };

    let mut chars = input.chars();
    let first = chars.next().unwrap();
    let current_value = (current_value + (first as u8 as usize)) * 17 % 256;

    run_hash_algorithm(chars.collect::<String>(), current_value)
}

pub fn solve() {
    // rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    let file = File::open("src/day15/initialization_sequence.txt").expect("💣");
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap().unwrap().to_owned();

    let steps = line.split(",").collect::<Vec<&str>>();
    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];

    for step in steps.iter() {
        let parts = step.split(|c| c == '=' || c == '-').filter(|part| !part.is_empty()) .collect::<Vec<&str>>();
        let box_i = run_hash_algorithm(parts[0].to_string(), 0);
        let label = parts[0].to_string();

        if parts.len() == 1 {
            boxes[box_i].retain(|lens| lens.label != label);
        } else {
            let focal_length = parts[1].parse::<usize>().unwrap();

            if let Some(existing_lens_i) = boxes[box_i].iter().position(|lens| lens.label == label) {
                boxes[box_i][existing_lens_i].focal_length = focal_length;
            } else {
                boxes[box_i].push(Lens { label, focal_length });
            }
        }
    };

    let sum = boxes.iter().enumerate().fold(0, |acc, (box_i, current_box)| {
        acc + current_box.iter().enumerate().fold(0, |acc, (lens_i, lens)| {
            acc + (1 + box_i) * (1 + lens_i) * lens.focal_length
        })
    });

    println!("Sum: {}", sum);
}
