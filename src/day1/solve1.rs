use crate::util::read_lines;

pub fn solve() {
    let mut sum: u32 = 0;

    for line in read_lines("src/day1/calibration.txt").unwrap() {
        let line = line.unwrap();
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

        sum += digits[0] * 10 + digits[digits.len() - 1];
    }
    println!("Sum: {}", sum);
}
