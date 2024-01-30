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

fn main() {
    let start = Instant::now();

    day9::solve2::solve();

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
