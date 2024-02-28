use crate::util::read_lines;

pub fn solve() {
    let universe: Vec<Vec<char>> = read_lines("src/day11/image.txt").unwrap().map(|line| {
        line.expect("Booom! 💣").chars().collect()
    }).collect();

    let mut empty_col_indices: Vec<usize> = Vec::new();
    let mut empty_row_indices: Vec<usize> = Vec::new();

    for x in 0..universe[0].len() {
        if universe.iter().all(|row| row[x] == '.') {
            empty_col_indices.push(x);
        }
    }

    for y in 0..universe.len() {
        if universe[y].iter().all(|char| char == &'.') {
            empty_row_indices.push(y);
        }
    }

    let expansion_factor = 1000000;
    let mut galaxy_positions: Vec<(usize, usize)> = Vec::new();

    for y in 0..universe.len() {
        for x in 0..universe[y].len() {
            if universe[y][x] != '.' {
                galaxy_positions.push((x, y));
            }
        }
    }

    let mut total_length = 0;

    for i in 0..galaxy_positions.len() {
        let pos_a = galaxy_positions[i];
        for j in i..galaxy_positions.len() {
            let pos_b = galaxy_positions[j];
            if pos_a == pos_b {
                continue;
            }
            let (mut x1, mut y1) = pos_a;
            let (mut x2, mut y2) = pos_b;

            if x1 > x2 {
                std::mem::swap(&mut x1, &mut x2);
            }

            if y1 > y2 {
                std::mem::swap(&mut y1, &mut y2);
            }

            let num_cols_to_expand = empty_col_indices.iter().filter(|i| *i > &x1 && *i < &x2).count();
            let num_rows_to_expand = empty_row_indices.iter().filter(|i| *i > &y1 && *i < &y2).count();

            total_length += x2 - x1 - num_cols_to_expand + expansion_factor * num_cols_to_expand
                          + y2 - y1 - num_rows_to_expand + expansion_factor * num_rows_to_expand;
        }
    }
    println!("Sum: {}", total_length);
}
