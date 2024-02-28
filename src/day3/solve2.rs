use crate::util::read_lines;

struct PositionedPartNumber {
    pos: (usize, usize),
    number: String,
}

fn create_and_store_new_positioned_part_number(current_part_number: String, positioned_part_numbers: &mut Vec<PositionedPartNumber>, x_pos: usize, y_pos: usize) -> String {
    if current_part_number.len() > 0 {
        positioned_part_numbers.push(PositionedPartNumber {
            pos: (x_pos - current_part_number.len(), y_pos),
            number: current_part_number,
        });
        return String::new();
    }
    current_part_number
}

pub fn solve() {
    let mut y_pos = 0;
    let mut positioned_part_numbers: Vec<PositionedPartNumber> = Vec::new();
    let mut potential_gear_positions: Vec<(usize, usize)> = Vec::new();

    for line in read_lines("src/day3/schematic.txt").unwrap() {
        let line = line.unwrap();
        let mut x_pos = 0;
        let mut current_part_number = String::new();

        for character in line.chars() {
            if character.is_ascii_digit() {
                current_part_number.push(character);
            } else {
                if character == '*' {
                    potential_gear_positions.push((x_pos, y_pos));
                }
                current_part_number = create_and_store_new_positioned_part_number(current_part_number, &mut positioned_part_numbers, x_pos, y_pos);
            }
            x_pos += 1;
        }
        create_and_store_new_positioned_part_number(current_part_number, &mut positioned_part_numbers, x_pos, y_pos);

        y_pos += 1;
    }

    let mut sum_gear_ratios: u32 = 0;

    for gear_position in &potential_gear_positions {
        let (gear_x_pos, gear_y_pos) = gear_position;
        let mut gear_parts: Vec<u32> = Vec::new();

        for positioned_part_number in &positioned_part_numbers {
            let PositionedPartNumber { pos, number } = positioned_part_number;
            let (x_pos, y_pos) = pos;

            if *gear_y_pos >= y_pos.saturating_sub(1) && *gear_y_pos <= y_pos + 1 {
                if *gear_x_pos >= x_pos.saturating_sub(1) && *gear_x_pos <= x_pos + number.len() {
                    gear_parts.push(number.parse::<u32>().unwrap());
                }
            }
        }

        if gear_parts.len() == 2 {
            sum_gear_ratios += gear_parts.iter().product::<u32>()
        }
    }
    println!("Sum: {}", sum_gear_ratios);
}
