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
    pub mod solve2_brute_force_parallell;
    pub mod solve2_reverse;
}

fn main() {
    let start = Instant::now();

    day5::solve2_reverse::solve();

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
