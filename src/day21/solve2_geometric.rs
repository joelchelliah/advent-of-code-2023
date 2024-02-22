use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;

type Pos = (isize, isize);
type NeighborMap = HashMap<(Pos, i32), HashSet<Pos>>;

// Geometric solution from:
// https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21

fn count_plots(map: &Vec<Vec<char>>, start_pos: Pos, steps: i32) -> usize {
    let mut queue: VecDeque<(Pos, i32)> = VecDeque::new();
    let mut visited: HashMap<Pos, i32> = HashMap::new();

    queue.push_back((start_pos, 0));

    while let Some((pos, steps_taken)) = queue.pop_front() {
        if visited.contains_key(&pos) { continue; }
        if steps_taken > steps { break; }

        visited.insert(pos, steps_taken);

        for (x, y) in vec![
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1)
        ].iter() {
            if *x >= 0 && *y >= 0 &&
               *x < map[0].len() as isize && *y < map.len() as isize &&
               map[*y as usize][*x as usize] != '#' &&
               !visited.contains_key(&(*x, *y)) {
                queue.push_back(((*x, *y), steps_taken + 1));
            }
        }
    }

    let even_corners = visited
        .values()
        .filter(|num_steps| **num_steps % 2 == 0 && **num_steps > 65)
        .count();
    let odd_corners = visited
        .values()
        .filter(|num_steps| **num_steps % 2 == 1 && **num_steps > 65)
        .count();

    let num_maps: usize = ((steps as usize - (map.len() / 2)) / map.len()) as usize;
    let even_input_squares = num_maps * num_maps;
    let odd_input_squares = (num_maps + 1) * (num_maps + 1);

    return odd_input_squares * visited.values().filter(|v| **v % 2 == 1).count()
        + even_input_squares * visited.values().filter(|v| **v % 2 == 0).count()
        - ((num_maps + 1) * odd_corners)
        + (num_maps * even_corners);
}

pub fn solve() {
    let file = File::open("src/day21/map.txt").expect("💣");
    let reader = BufReader::new(file);

    let map: Vec<Vec<char>> = reader.lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut start = (0, 0);

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                start = (x as isize, y as isize);
            }
        }
    }

    let count = count_plots(&map, start, 26501365);

    println!("Sum: {}", count);
}
