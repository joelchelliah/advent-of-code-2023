use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;

type Pos = (isize, isize);
type NeighborMap = HashMap<(Pos, i32), HashSet<Pos>>;

fn count_plots(map: &Vec<Vec<char>>, start_pos: Pos, steps: i32) -> i32 {
    let mut queue: VecDeque<(Pos, i32)> = VecDeque::new();
    let mut visited: HashMap<Pos, i32> = HashMap::new();

    queue.push_back((start_pos, 0));

    while let Some((pos, steps_taken)) = queue.pop_front() {
        if visited.contains_key(&pos) { continue; }
        if steps_taken > steps { break; }

        visited.insert(pos, steps_taken);

        for new_pos in vec![
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1)
        ].iter() {
            if new_pos.0 >= 0 &&
               new_pos.1 >= 0 &&
               new_pos.0 < map[0].len() as isize &&
               new_pos.1 < map.len() as isize &&
               map[new_pos.1 as usize][new_pos.0 as usize] != '#' &&
               !visited.contains_key(&new_pos) {
                queue.push_back((*new_pos, steps_taken + 1));
            }
        }
    }

    return visited
        .values()
        .filter(|num_steps| **num_steps <= steps && **num_steps % 2 == 0)
        .count() as i32;


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
                break;
            }
        }
    }

    let count = count_plots(&map, start, 64);

    println!("Sum: {}", count);
}
