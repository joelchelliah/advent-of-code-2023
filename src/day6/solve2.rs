use crate::util::read_lines;

pub fn solve() {
    let mut time: String = String::new();
    let mut distance: String = String::new();

    for line in read_lines("src/day6/record.txt").unwrap() {
        let line = line.unwrap();
        let line_parts: Vec<&str> = line.split(":").collect::<Vec<&str>>();

        if line_parts[0].starts_with("Time") {
            time = line_parts[1].trim().split(" ").collect::<Vec<&str>>()
                .iter()
                .filter(|digits| !digits.is_empty())
                .map(|digits| digits.to_string())
                .collect();
        } else {
            distance = line_parts[1].trim().split(" ").collect::<Vec<&str>>()
                .iter()
                .filter(|digits: &&&str| !digits.is_empty())
                .map(|digits| digits.to_string())
                .collect();
        }
    }

    let max_time = time.parse::<u128>().unwrap();
    let record_distance = distance.parse::<u128>().unwrap();
    let mut num_record_breaks = 0;

    for hold_time in 1..max_time {
        let run_time = max_time - hold_time;
        let distance = hold_time * run_time;

        if distance > record_distance {
            num_record_breaks += 1;
        }
    }

    println!("Answer: {}", num_record_breaks);
}
