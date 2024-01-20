use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn solve() {
    let file = File::open("src/day2/games.txt").expect("💣");
    let reader = BufReader::new(file);

    let mut sum_powers: u32 = 0;

    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.trim().is_empty() {
            break;
        }

        // Game 4
        // 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        let parts: Vec<&str> = line.split(":").collect();
        let game_id: u32 = parts[0].trim().split_whitespace().nth(1).unwrap().parse().unwrap();
        let game_results = parts[1].trim().split(";").collect::<Vec<&str>>();

        let mut max_red: u32 = 0;
        let mut max_green: u32 = 0;
        let mut max_blue: u32 = 0;

        // 1 green, 3 red, 6 blue
        // 3 green, 6 red
        // 3 green, 15 blue, 14 red
        for game_part in game_results {

            // 1 green
            // 3 red
            // 6 blue
            for cube_count in game_part.split(",").collect::<Vec<&str>>() {
                let num_and_color: Vec<&str> = cube_count.trim().split(" ").collect();
                let [num_string, color] = match num_and_color.as_slice() {
                    [num_string, color] => [num_string.trim(), color.trim()],
                    _ => panic!("Destructuring failed! 💣"),
                };
                let num = num_string.parse::<u32>().unwrap();

                if color == "red" && num > max_red {
                    max_red = num;
                } else if color == "green" && num > max_green {
                    max_green = num;
                } else if color == "blue" && num > max_blue {
                    max_blue = num;
                }
            }
        }

        sum_powers += (max_red * max_green * max_blue);

    }

    println!("Sum: {}", sum_powers);
}
