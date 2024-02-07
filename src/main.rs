#![allow(dead_code)]

use std::time::Instant;

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

fn main() {
    let start = Instant::now();

    day16::solve2::solve();

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
