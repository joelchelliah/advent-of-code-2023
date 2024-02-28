use crate::util::read_lines;

pub fn solve() {
    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;

    let mut sum_ids: u32 = 0;

    for line in read_lines("src/day2/games.txt").unwrap() {
        let line = line.unwrap();
        let (left, right) = line.split_once(":").unwrap();
        let game_id: u32 = left.split_whitespace().nth(1).unwrap().parse().unwrap();
        let game_results = right.split(";");

        let mut is_valid_game = true;

        'game: for result in game_results {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for cube_count in result.split(",") {
                let (num, color) = cube_count.trim().split_once(" ").unwrap();
                let num: u32 = num.parse().unwrap();

                match color {
                    "red" => red = num,
                    "green" => green = num,
                    "blue" => blue = num,
                    _ => continue,
                }
            }

            if red > max_red || green > max_green || blue > max_blue {
                is_valid_game = false;
                break 'game;
            }
        }

        if is_valid_game {
            sum_ids += game_id;
        }
    }

    println!("Sum: {}", sum_ids);
}
