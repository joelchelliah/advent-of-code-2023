use crate::util::read_lines;

fn get_numbers(input: &str) -> Vec<&str> {
    input.trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>()
}

fn balance_and_combine_vectors<T: Default + Clone + std::ops::Add<Output = T>>(mut vec1: Vec<T>, mut vec2: Vec<T>) -> Vec<T> {
    if vec2.len() > vec1.len() {
        vec1.extend(vec![T::default(); vec2.len() - vec1.len()]);
    } else {
        vec2.extend(vec![T::default(); vec1.len() - vec2.len()]);
    }

    vec1.iter().zip(vec2.iter())
        .map(|(a, b)| a.clone() + b.clone())
        .collect()
}

pub fn solve() {
    let mut total_num_scratch_cards = 0;
    let mut copies_of_upcoming_scratch_cards: Vec<usize> = Vec::new();

    for line in read_lines("src/day4/scratch_cards.txt").unwrap() {
        let line = line.unwrap();
        let mut current_copies: usize = 1;

        if copies_of_upcoming_scratch_cards.len() > 0 {
            current_copies += copies_of_upcoming_scratch_cards[0];
            copies_of_upcoming_scratch_cards = copies_of_upcoming_scratch_cards[1..].to_vec();
        }

        total_num_scratch_cards += current_copies;

        let number_lists: Vec<&str> = line.split(":").collect::<Vec<&str>>()[1].split("|").collect();
        let winning_numbers: Vec<&str> = get_numbers(number_lists[0]);
        let playing_numbers: Vec<&str> = get_numbers(number_lists[1]);

        let num_matching_numbers: usize = playing_numbers
            .iter()
            .fold(0, |acc, number| if winning_numbers.contains(number) { acc + 1 } else { acc });
        let new_copies_of_upcoming_scratch_cards: Vec<usize> = vec![current_copies; num_matching_numbers];

        copies_of_upcoming_scratch_cards = balance_and_combine_vectors(copies_of_upcoming_scratch_cards, new_copies_of_upcoming_scratch_cards);
    }

    println!("Sum: {}", total_num_scratch_cards);
}
