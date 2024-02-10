use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Edge {
    pos: (usize, usize),
    color: String,
}

// https://en.wikipedia.org/wiki/Shoelace_formula
fn calculate_area(edges: &[Edge]) -> f64 {
    let mut sum = 0.0;

    for i in 0..edges.len() {
        let this_edge = &edges[i];
        let next_edge = &edges[(i + 1) % edges.len()];

        sum += this_edge.pos.0 as f64 * next_edge.pos.1 as f64;
        sum -= this_edge.pos.1 as f64 * next_edge.pos.0 as f64;
    }

    (sum.abs()) / 2.0
}

// https://en.wikipedia.org/wiki/Pick%27s_theorem
fn get_num_enclosed_points(edges: &[Edge]) -> f64 {
    let area = calculate_area(edges);
    let num_points_on_path = edges.len() as f64;

    (area - num_points_on_path / 2.0 + 1.0).ceil()
}

pub fn solve() {
    let file = File::open("src/day18/dig_plan.txt").expect("💣");
    let reader = BufReader::new(file);

    let mut last_corner = (999, 999);
    let dig_plan_edges: Vec<Edge> = reader.lines().flat_map(|line| {
        let line = line.unwrap();
        let parts = line.split(" ").collect::<Vec<&str>>();
        let (direction, steps, color) = (parts[0], parts[1], parts[2]);
        let (x, y) = last_corner;

        let next_corner = match direction {
            "R" => (x + steps.parse::<usize>().unwrap(), y),
            "L" => (x - steps.parse::<usize>().unwrap(), y),
            "D" => (x, y + steps.parse::<usize>().unwrap()),
            "U" => (x, y - steps.parse::<usize>().unwrap()),
            _ => panic!("😱 Unknown direction: {}!", direction),
        };

        let mut edges = Vec::new();
        if direction == "R" {
            for new_x in (x + 1)..=next_corner.0 {
                edges.push(Edge { pos: (new_x, y), color: color.to_string(), });
            }
        } else if direction == "L" {
            for new_x in (next_corner.0..x).rev() {
                edges.push(Edge { pos: (new_x, y), color: color.to_string(), });
            }
        } else if direction == "D" {
            for new_y in (y + 1)..=next_corner.1 {
                edges.push(Edge { pos: (x, new_y), color: color.to_string(), });
            }
        } else {
            for new_y in (next_corner.1..y).rev() {
                edges.push(Edge { pos: (x, new_y), color: color.to_string(), });
            }
        }

        last_corner = next_corner;
        edges
    }).collect();

    let sum = dig_plan_edges.len() as f64 + get_num_enclosed_points(&dig_plan_edges);

    println!("Sum: {}", sum);
}
