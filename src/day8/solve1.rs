use std::io::{BufRead, BufReader};
use std::fs::File;

struct Node {
    from: String,
    left: String,
    right: String,
}

fn find_node<'a>(nodes: &'a Vec<Node>, pos: &str) -> &'a Node {
    nodes.iter().find(|node| node.from == pos).unwrap()
}

pub fn solve() {
    // LLR

    // AAA = (BBB, BBB)
    // BBB = (AAA, ZZZ)
    // ZZZ = (ZZZ, ZZZ)
    let file = File::open("src/day8/network.txt").expect("💣");
    let reader = BufReader::new(file);

    let mut directions: Vec<char> = Vec::new();
    let mut nodes: Vec<Node> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Nuuuu! 💣");
        if line.trim().is_empty() {
            if nodes.is_empty() {
                continue;
            } else {
                break;
            }
        }

        if directions.is_empty() {
            // LLR
            directions = line.chars().collect();
        } else {
            let parts: Vec<&str> = line.split("=").collect();
            // AAA
            let from = parts[0].trim().to_string();
            // [BBB, BBB]
            let [left, right] = {
                let to = parts[1].trim();
                let to = &to[1..to.len()-1].split(",").collect::<Vec<&str>>();
                [to[0].trim().to_string(), to[1].trim().to_string()]
            };

            nodes.push(Node { from, left, right });
        }
    }

    let mut directions = directions.iter().cycle();
    let mut pos = "AAA";
    let mut total_steps = 1;

    loop {
        let direction = directions.next().unwrap();
        if direction == &'L' {
            pos = &find_node(&nodes, pos).left;
        } else {
            pos = &find_node(&nodes, pos).right;
        }

        if pos == "ZZZ" {
            break;
        } else {
            total_steps += 1;
        }
    }

    println!("Answer: {}", total_steps);
}
