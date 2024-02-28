use crate::util::read_lines;

pub fn solve() {
    let mut sum_powers: u32 = 0;

    for line in read_lines("src/day2/games.txt").unwrap() {
        let line = line.unwrap();
        let game_results = line.split_once(":").unwrap().1.split(";");

        let mut max_red: u32 = 0;
        let mut max_green: u32 = 0;
        let mut max_blue: u32 = 0;

        for result in game_results {
            for cube_count in result.split(",") {
                let (num, color) = cube_count.trim().split_once(" ").unwrap();
                let num = num.trim().parse::<u32>().unwrap();

                if color == "red" && num > max_red {
                    max_red = num;
                } else if color == "green" && num > max_green {
                    max_green = num;
                } else if color == "blue" && num > max_blue {
                    max_blue = num;
                }
            }
        }
        sum_powers += max_red * max_green * max_blue;

    }
    println!("Sum: {}", sum_powers);
}
