use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

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

fn rotate_clockwise(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rotated_platform = platform.clone();
    let n = platform.len();

    for i in 0..n {
        for j in 0..n {
            rotated_platform[j][n-i-1] = platform[i][j];
        }
    }
    rotated_platform
}

fn spin_cycle(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut spun_platform = platform.clone();

    for _ in 0..4 {
        spun_platform = tilt_north(&spun_platform, 1);
        spun_platform = rotate_clockwise(&spun_platform);
    }

    spun_platform
}

pub fn solve() {
    // O....#....
    // O.OO#....#
    // .....##...
    let file = File::open("src/day14/positions.txt").expect("💣");
    let reader = BufReader::new(file);

    let mut platform: Vec<Vec<char>> = reader.lines()
        .map(|line| line.expect("Nuuuu! 💣").chars().collect())
        .collect();


    let mut cache = HashMap::new();
    let mut platform_states: Vec<Vec<Vec<char>>> = Vec::new();
    let iterations = 1000000000;
    let mut cycle_detected_at = 0;
    let mut cycle_start_index = 0;

    for i in 0..200 {
        if cache.contains_key(&platform) {
            if cycle_detected_at == 0 {
                cycle_detected_at = i;
                cycle_start_index = cache[&platform];
            }
        } else {
            cache.insert(platform.clone(), i);
            platform_states.push(platform.clone());
            cycle_detected_at = 0;
        }
        platform = spin_cycle(&platform);
    }

    let cycle_length = cycle_detected_at - cycle_start_index;
    let end_state_index = ((iterations - cycle_start_index) % cycle_length) + cycle_start_index;

    platform = platform_states[end_state_index].clone();

    let mut total_load = 0;
    for y in 0..platform.len() {
        let num_rocks = platform[y].iter().filter(|x| **x == 'O').count();
        let rev_pos = platform.len() - y;

        total_load += rev_pos * num_rocks;
    }

    println!("Sum: {}", total_load);
}
