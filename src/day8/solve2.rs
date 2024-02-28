use crate::util::read_lines;

struct Node {
    from: String,
    left: String,
    right: String,
}

fn find_node<'a>(nodes: &'a Vec<Node>, pos: &str) -> &'a Node {
    nodes.iter().find(|node| node.from == pos).unwrap()
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i128, b: i128) -> i128 {
    (a * b) / gcd(a, b)
}

pub fn solve() {
    let mut directions: Vec<char> = Vec::new();
    let mut nodes: Vec<Node> = Vec::new();

    for line in read_lines("src/day8/network.txt").unwrap() {
        let line = line.unwrap();
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
    let positions: Vec<&str> = nodes.iter().map(|node| node.from.as_str())
                                               .filter(|pos| pos.ends_with("A"))
                                               .collect();

    let total_steps_per_position: Vec<i32> = positions.iter().map(|&pos| {
        let mut steps = 0;
        let mut pos = pos;
        loop {
            let direction = directions.next().unwrap();
            if direction == &'L' {
                pos = find_node(&nodes, pos).left.as_str();
            } else {
                pos = find_node(&nodes, pos).right.as_str();
            };
            steps += 1;

            if pos.ends_with("Z") {
                break;
            }
        }
        steps
    }).collect();

    let total_steps: i128 =
        total_steps_per_position.iter()
                                .fold(1, |acc, &steps| lcm(acc, steps as i128));

    println!("Answer: {}", total_steps);
}
