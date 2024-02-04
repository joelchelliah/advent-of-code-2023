use std::io::{BufRead, BufReader};
use std::fs::File;

fn tilt_north(platform: &Vec<Vec<char>>, y: usize) -> Vec<Vec<char>> {
    if y == platform.len() { return platform.clone() };

    let mut new_platform: Vec<Vec<char>> = platform.clone();

    for x in 0..new_platform[y].len() {
        let mut current_pos_y = y;

        for above_pos_y in (0..y).rev() {
            if new_platform[current_pos_y][x] == 'O' && new_platform[above_pos_y][x] == '.' {
                new_platform[above_pos_y][x] = 'O';
                new_platform[current_pos_y][x] = '.';
                current_pos_y = above_pos_y;
            } else {
                break;
            }
        }
    }
    tilt_north(&new_platform, y + 1)
}

pub fn solve() {
    // O....#....
    // O.OO#....#
    // .....##...
    let file = File::open("src/day14/positions.txt").expect("💣");
    let reader = BufReader::new(file);

    let platform: Vec<Vec<char>> = reader.lines()
        .map(|line| line.expect("Nuuuu! 💣").chars().collect())
        .collect();

    let platform = tilt_north(&platform, 1);
    let mut total_load = 0;

    for y in 0..platform.len() {
        let num_rocks = platform[y].iter().filter(|x| **x == 'O').count();
        let rev_pos = platform.len() - y;

        total_load += rev_pos * num_rocks;
    }

    println!("Sum: {}", total_load);
}
