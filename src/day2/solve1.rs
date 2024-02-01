use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn solve() {
    let file = File::open("src/day2/games.txt").expect("💣");
    let reader = BufReader::new(file);

    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;

    let mut sum_ids: u32 = 0;

    // Game 8: 6 green, 7 blue; 9 green, 6 blue; 7 blue, 1 red, 3 green
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.trim().is_empty() {
            break;
        }

        // Game 8
        // 6 green, 7 blue; 9 green, 6 blue; 7 blue, 1 red, 3 green
        let parts: Vec<&str> = line.split(":").collect();
        let game_id: u32 = parts[0].trim().split_whitespace().nth(1).unwrap().parse().unwrap();
        let game_results = parts[1].trim().split(";").collect::<Vec<&str>>();
        let mut is_valid_game = true;

        // 6 green, 7 blue
        // 9 green, 6 blue
        // 7 blue, 1 red, 3 green
        for game_part in game_results {
            if !is_valid_game {
                break;
            }

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            // 6 green
            // 7 blue
            for cube_count in game_part.split(",").collect::<Vec<&str>>() {
                let num_and_color: Vec<&str> = cube_count.trim().split(" ").collect();

                if num_and_color[1].trim() == "red" {
                    red = num_and_color[0].trim().parse::<u32>().unwrap();
                } else if num_and_color[1].trim() == "green" {
                    green = num_and_color[0].trim().parse::<u32>().unwrap();
                } else if num_and_color[1].trim() == "blue" {
                    blue = num_and_color[0].trim().parse::<u32>().unwrap();
                }
            }

            if red > max_red || green > max_green || blue > max_blue {
                is_valid_game = false;
                break;
            }
        }

        if is_valid_game {
            sum_ids += game_id;
        }
    }

    println!("Sum: {}", sum_ids);
}
