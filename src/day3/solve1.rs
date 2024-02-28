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
    let mut symbol_positions: Vec<(usize, usize)> = Vec::new();
    let mut sum_part_numbers: u32 = 0;


    for line in read_lines("src/day3/schematic.txt").unwrap() {
        let line = line.unwrap();
        let mut x_pos = 0;
        let mut current_part_number = String::new();

        for character in line.chars() {
            if character.is_ascii_digit() {
                current_part_number.push(character);
            } else {
                if character != '.' {
                    symbol_positions.push((x_pos, y_pos));
                }
                current_part_number = create_and_store_new_positioned_part_number(current_part_number, &mut positioned_part_numbers, x_pos, y_pos);
            }
            x_pos += 1;
        }
        create_and_store_new_positioned_part_number(current_part_number, &mut positioned_part_numbers, x_pos, y_pos);

        y_pos += 1;
    }

    for positioned_part_number in positioned_part_numbers {
        let PositionedPartNumber { pos, number } = positioned_part_number;
        let (x_pos, y_pos) = pos;

        for symbol_position in &symbol_positions {
            let (symbol_x_pos, symbol_y_pos) = symbol_position;

            if *symbol_y_pos >= y_pos.saturating_sub(1) && *symbol_y_pos <= y_pos + 1 {
                if *symbol_x_pos >= x_pos.saturating_sub(1) && *symbol_x_pos <= x_pos + number.len() {
                    sum_part_numbers += number.parse::<u32>().unwrap();
                    break;
                }
            }
        }
    }

    println!("Sum: {}", sum_part_numbers);
}
