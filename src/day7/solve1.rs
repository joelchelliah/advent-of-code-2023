use std::io::{BufRead, BufReader};
use std::fs::File;

type GroupedLabels = Vec<Vec<char>>;

fn group_labels(hand: &String) -> GroupedLabels {
    let mut chars: Vec<char> = hand.chars().collect();
    chars.sort();

    let mut iter = chars.into_iter();
    let mut grouped: GroupedLabels = Vec::new();

    if let Some(first) = iter.next() {
        let mut current_group = vec![first];

        for ch in iter {
            if ch == *current_group.last().unwrap() {
                current_group.push(ch);
            } else {
                grouped.push(current_group);
                current_group = vec![ch];
            }
        }

        grouped.push(current_group);
    }
    grouped
}

fn is_five_of_a_kind(group: &GroupedLabels) -> bool {
    group.len() == 1
}

fn is_four_of_a_kind(group: &GroupedLabels) -> bool {
    group[0].len() == 4 || group[1].len() == 4
}

fn is_full_house(group: &GroupedLabels) -> bool {
    group[0].len() == 3 && group[1].len() == 2 || group[0].len() == 2 && group[1].len() == 3
}

fn is_three_of_a_kind(group: &GroupedLabels) -> bool {
    group[0].len() == 3 || group[1].len() == 3 || group[2].len() == 3
}

fn is_two_pair(group: &GroupedLabels) -> bool {
    group.iter().filter(|labels| labels.len() == 2).count() == 2
}

fn is_one_pair(group: &GroupedLabels) -> bool {
    group.len() == 4
}

fn score_hand(hand: &String) -> i32 {
    let group = group_labels(hand);

    if is_five_of_a_kind(&group) {
        return 7;
    } else if is_four_of_a_kind(&group) {
        return 6;
    } else if is_full_house(&group) {
        return 5;
    } else if is_three_of_a_kind(&group) {
        return 4;
    } else if is_two_pair(&group) {
        return 3;
    } else if is_one_pair(&group) {
        return 2;
    } else {
        return 1;
    }
}

fn score_label(label: char) -> i32 {
    if label.is_digit(10) {
        return label.to_digit(10).unwrap() as i32;
    } else if label == 'T' {
        return 10;
    } else if label == 'J' {
        return 11;
    } else if label == 'Q' {
        return 12;
    } else if label == 'K' {
        return 13;
    } else if label == 'A' {
        return 14;
    } else {
        panic!("Unknown label: {}", label);
    }
}

pub fn solve() {
    // 32T3K 765
    // T55J5 684
    // KK677 28
    let file = File::open("src/day7/round.txt").expect("💣");
    let reader = BufReader::new(file);

    let mut hands: Vec<String> = Vec::new();
    let mut bids: Vec<i32> = Vec::new();

    // 32T3K 765
    for line in reader.lines() {
        let line = line.expect("Nuuuu! 💣");
        if line.trim().is_empty() {
            break;
        }

        let hand_and_bid: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();

        hands.push(hand_and_bid[0].clone());
        bids.push(hand_and_bid[1].parse::<i32>().unwrap());
    }

    let mut reverse_sorted_hands: Vec<String> = hands.iter().map(|hand| hand.clone()).collect();
    reverse_sorted_hands.sort_by(|a, b| {
        let score_comparison = score_hand(a).cmp(&score_hand(b));
        if score_comparison == std::cmp::Ordering::Equal {
            for i in 0..a.len() {
                let a_label = a.chars().nth(i).unwrap();
                let b_label = b.chars().nth(i).unwrap();
                let label_comparison = score_label(a_label).cmp(&score_label(b_label));
                if label_comparison != std::cmp::Ordering::Equal {
                    return label_comparison;
                }
            }
            return std::cmp::Ordering::Equal;
        } else {
            score_comparison
        }
    });

    let mut total_winnings = 0;
    for sorted_index in 0..reverse_sorted_hands.len() {
        let rank = sorted_index as i32 + 1;
        let unsorted_index = hands.iter().position(|h| *h == reverse_sorted_hands[sorted_index]).unwrap();
        let bid = bids[unsorted_index];

        total_winnings += bid * rank;
    }

    println!("Answer: {}", total_winnings);
}
