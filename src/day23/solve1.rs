use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::fs::File;

type Pos = (usize, usize);

fn walk(map: &Vec<Vec<char>>, start: Pos, end: Pos, steps: i32, mut visited: HashSet<Pos>) -> i32 {
    if start == end { return steps; }
    if visited.contains(&start) { return 0; }

    visited.insert(start);

    let (x, y) = start;
    if map[y][x] == '>' {
        return walk(map, (x+1, y), end, steps + 1, visited);
    } else if map[y][x] == '<' {
        return walk(map, (x-1, y), end, steps + 1, visited);
    } else if map[y][x] == '^' {
        return walk(map, (x, y-1), end, steps + 1, visited);
    } else if map[y][x] == 'v' {
        return walk(map, (x, y+1), end, steps + 1, visited);
    }

    let (x, y) = (start.0 as isize, start.1 as isize);
    let next_steps = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].iter()
        .filter(|(x, y)|
            *x >= 0 &&
            *y >= 0 &&
            *x < map[0].len() as isize &&
            *y < map.len() as isize &&
            map[*y as usize][*x as usize] != '#'
        )
        .map(|(x, y)| (*x as usize, *y as usize))
        .collect::<Vec<(usize, usize)>>();

    next_steps.iter()
        .map(|pos| walk(map, *pos, end, steps + 1, visited.clone()))
        .max().unwrap()
}

pub fn solve() {
    let file = File::open("src/day23/hiking_trails.txt").expect("💣");
    let reader = BufReader::new(file);

    let map: Vec<Vec<char>> = reader.lines()
        .map(|line| line.unwrap().chars()
            .map(|c| c).collect()).collect();
    let start = (1, 0);
    let end = (map[0].len() - 2, map.len() - 1);
    let count = walk(&map, start, end, 0, HashSet::new());

    println!("Sum: {}", count);
}
