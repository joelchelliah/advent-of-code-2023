use std::collections::HashSet;

use crate::util::read_lines;

fn fill_numbers<'a>(input: &'a str, set: &mut HashSet<String>) {
    set.clear();
    input.trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .for_each(|x| { set.insert(x.to_string()); });
}

pub fn solve() {
    let mut total_points = 0;
    let mut winning_numbers = HashSet::new();
    let mut playing_numbers = HashSet::new();

    for line in read_lines("src/day4/scratch_cards.txt").unwrap() {
        let line = line.unwrap();
        let number_lists: Vec<&str> = line.split(":").collect::<Vec<&str>>()[1].split("|").collect();
        fill_numbers(number_lists[0], &mut winning_numbers);
        fill_numbers(number_lists[1], &mut playing_numbers);
        let mut points = 0;

        for number in &playing_numbers {
            if winning_numbers.contains(number) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        total_points += points;
    }

    println!("Sum: {}", total_points);
}
