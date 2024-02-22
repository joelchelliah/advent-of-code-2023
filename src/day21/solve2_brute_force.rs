use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;

type Pos = (isize, isize);
type NeighborMap = HashMap<(Pos, i32), HashSet<Pos>>;

fn place_within_range(mut n: isize, max: usize) -> isize {
    let max = max as isize;
    while n < 0 { n += max; }
    while n >= max { n -= max; }
    n
}

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
            if visited.contains_key(&new_pos) { continue; }

            let mod_x =  place_within_range(new_pos.0, map[0].len());
            let mod_y =  place_within_range(new_pos.1, map.len());

            if map[mod_y as usize][mod_x as usize] != '#' {
                queue.push_back((*new_pos, steps_taken + 1));
            }
        }
    }

    return visited
        .values()
        .filter(|num_steps| **num_steps <= steps && **num_steps % 2 != 0)
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
            }
        }
    }

    let count = count_plots(&map, start, 26501365);

    println!("Sum: {}", count);
}
