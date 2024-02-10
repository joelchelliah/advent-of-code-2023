use std::io::{BufRead, BufReader};
use std::fs::File;

// https://en.wikipedia.org/wiki/Shoelace_formula
fn calculate_area(edges: &[(u32, u32)]) -> f64 {
    let mut sum = 0.0;

    for i in 0..edges.len() {
        let this_edge = &edges[i];
        let next_edge = &edges[(i + 1) % edges.len()];

        sum += this_edge.0 as f64 * next_edge.1 as f64;
        sum -= this_edge.1 as f64 * next_edge.0 as f64;
    }

    (sum.abs()) / 2.0
}

// https://en.wikipedia.org/wiki/Pick%27s_theorem
fn get_num_enclosed_points(edges: &[(u32, u32)]) -> f64 {
    let area = calculate_area(edges);
    let num_points_on_path = edges.len() as f64;

    (area - num_points_on_path / 2.0 + 1.0).ceil()
}

pub fn solve() {
    let file = File::open("src/day18/dig_plan.txt").expect("💣");
    let reader = BufReader::new(file);

    let mut last_corner = (9999999, 9999999);
    let dig_plan_edges: Vec<(u32, u32)> = reader.lines().flat_map(|line| {
        let line = line.unwrap();
        let parts = line.split("(").collect::<Vec<&str>>();

        let (x, y) = last_corner;

        let steps = u32::from_str_radix(parts[1][1..6].trim(), 16).unwrap();
        let (direction, next_corner) = match parts[1].chars().nth(6).unwrap() {
            '0' => ("R", (x + steps, y)),
            '1' => ("D", (x, y + steps)),
            '2' => ("L", (x - steps, y)),
            '3' => ("U", (x, y - steps)),
            _ => panic!("😱 Unknown direction!"),
        };

        let mut edges = Vec::new();
        if direction == "R" {
            for new_x in (x + 1)..=next_corner.0 {
                edges.push((new_x, y));
            }
        } else if direction == "L" {
            for new_x in (next_corner.0..x).rev() {
                edges.push((new_x, y));
            }
        } else if direction == "D" {
            for new_y in (y + 1)..=next_corner.1 {
                edges.push((x, new_y));
            }
        } else {
            for new_y in (next_corner.1..y).rev() {
                edges.push((x, new_y));
            }
        }

        last_corner = next_corner;
        edges
    }).collect();

    let sum = dig_plan_edges.len() as f64 + get_num_enclosed_points(&dig_plan_edges);

    println!("Sum: {}", sum);
}
