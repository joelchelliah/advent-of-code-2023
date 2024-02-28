#![allow(dead_code)]

use std::time::Instant;

pub mod util;

mod day1 {
    pub mod solve1;
    pub mod solve2;
}
mod day2 {
    pub mod solve1;
    pub mod solve2;
}
mod day3 {
    pub mod solve1;
    pub mod solve2;
}
mod day4 {
    pub mod solve1;
    pub mod solve2;
}
mod day5 {
    pub mod solve1;
    pub mod solve2_brute_force;
    pub mod solve2_brute_force_parallel;
    pub mod solve2_reverse;
}
mod day6 {
    pub mod solve1;
    pub mod solve2;
}
mod day7 {
    pub mod solve1;
    pub mod solve2;
}
mod day8 {
    pub mod solve1;
    pub mod solve2;
}
mod day9 {
    pub mod solve1;
    pub mod solve2;
}
mod day10 {
    pub mod solve1;
    pub mod solve2;
    pub mod solve2_picks_theorem;
}
mod day11 {
    pub mod solve1;
    pub mod solve2;
}
mod day12 {
    pub mod solve1;
    pub mod solve2;
}
mod day13 {
    pub mod solve1;
    pub mod solve2;
}
mod day14 {
    pub mod solve1;
    pub mod solve2;
}
mod day15 {
    pub mod solve1;
    pub mod solve2;
}
mod day16 {
    pub mod solve1;
    pub mod solve2;
}
mod day17 {
    pub mod solve1;
    pub mod solve2;
}
mod day18 {
    pub mod solve1;
    pub mod solve2;
}
mod day19 {
    pub mod solve1;
    pub mod solve2;
}
mod day20 {
    pub mod solve1;
    pub mod solve2;
}
mod day21 {
    pub mod solve1;
    pub mod solve2_brute_force;
    pub mod solve2_geometric;
}
mod day22 {
    pub mod solve1;
    pub mod solve2;
}
mod day23 {
    pub mod solve1;
    pub mod solve2;
}
mod day24 {
    pub mod solve1;
    pub mod solve2;
}
mod day25 {
    pub mod solve1_brute_force;
    pub mod solve1_kargers_algorithm;
}

fn main() {
    let start = Instant::now();

    day25::solve1_kargers_algorithm::solve();

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
