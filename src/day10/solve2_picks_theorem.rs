use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(PartialEq)]
#[derive(Clone)]
enum TileType {
    Ground,
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSW,
    BendSE,
    Start
}

#[derive(PartialEq)]
#[derive(Clone)]
struct Tile {
    tile_type: TileType,
    pos: (i32, i32),
}

fn create_tile(tile_type: TileType, pos: (i32, i32)) -> Tile {
    Tile {
        tile_type,
        pos,
    }
}

fn get_tile_at(map: &Vec<Vec<Tile>>, pos: (i32, i32)) -> &Tile {
    &map[pos.1 as usize][pos.0 as usize]
}

fn get_next_pos(tile: &Tile, prev_tile: &Tile) -> (i32, i32) {
    let mut positions = match tile.tile_type {
        TileType::Horizontal => vec![(tile.pos.0 - 1, tile.pos.1), (tile.pos.0 + 1, tile.pos.1)],
        TileType::Vertical => vec![(tile.pos.0, tile.pos.1 - 1), (tile.pos.0, tile.pos.1 + 1)],
        TileType::BendNE => vec![(tile.pos.0 + 1, tile.pos.1), (tile.pos.0, tile.pos.1 - 1)],
        TileType::BendNW => vec![(tile.pos.0 - 1, tile.pos.1), (tile.pos.0, tile.pos.1 - 1)],
        TileType::BendSW => vec![(tile.pos.0 - 1, tile.pos.1), (tile.pos.0, tile.pos.1 + 1)],
        TileType::BendSE => vec![(tile.pos.0 + 1, tile.pos.1), (tile.pos.0, tile.pos.1 + 1)],
        _ => panic!("Unknown tile type!")
    };

    if let Some(pos) = positions.iter().position(|&pos| pos == prev_tile.pos && pos != tile.pos) {
        positions.remove(pos);
    };
    positions[0]
}

fn get_tile_type(map: &Vec<Vec<Tile>>, tile: &Tile) -> TileType {
    let left_is_valid = tile.pos.0 - 1 >= 0 && is_valid_left_tile(get_tile_at(&map, (tile.pos.0 - 1, tile.pos.1)));
    let right_is_valid = tile.pos.0 + 1 < map[0].len() as i32 && is_valid_right_tile(get_tile_at(&map, (tile.pos.0 + 1, tile.pos.1)));
    let top_is_valid = tile.pos.1 - 1 >= 0 && is_valid_top_tile(get_tile_at(&map, (tile.pos.0, tile.pos.1 - 1)));
    let bottom_is_valid = tile.pos.1 + 1 < map.len() as i32 && is_valid_bottom_tile(get_tile_at(&map, (tile.pos.0, tile.pos.1 + 1)));

    match (left_is_valid, right_is_valid, top_is_valid, bottom_is_valid) {
        (true, true, false, false) => TileType::Horizontal,
        (false, false, true, true) => TileType::Vertical,
        (false, true, true, false) => TileType::BendNE,
        (true, false, true, false) => TileType::BendNW,
        (true, false, false, true) => TileType::BendSW,
        (false, true, false, true) => TileType::BendSE,
        _ => panic!("Unknown tile type!")
    }
}

fn count_distance(map: &Vec<Vec<Tile>>, tile: &Tile, prev_tile: &Tile, distance: i32, traversed_tiles: &mut Vec<Tile>) -> i32 {
    let next_pos = get_next_pos(&tile, prev_tile);
    let next_tile = get_tile_at(&map, next_pos);

    traversed_tiles.push(tile.clone());

    if next_tile.tile_type == TileType::Start {
        return (distance + 1) / 2;
    }

    count_distance(map, next_tile, tile, distance + 1, traversed_tiles)
}

fn is_valid_left_tile(tile: &Tile) -> bool {
    tile.tile_type == TileType::Horizontal ||
        tile.tile_type == TileType::BendNE ||
        tile.tile_type == TileType::BendSE
}

fn is_valid_right_tile(tile: &Tile) -> bool {
    tile.tile_type == TileType::Horizontal ||
        tile.tile_type == TileType::BendNW ||
        tile.tile_type == TileType::BendSW
}

fn is_valid_top_tile(tile: &Tile) -> bool {
    tile.tile_type == TileType::Vertical ||
        tile.tile_type == TileType::BendSE ||
        tile.tile_type == TileType::BendSW
}

fn is_valid_bottom_tile(tile: &Tile) -> bool {
    tile.tile_type == TileType::Vertical ||
        tile.tile_type == TileType::BendNE ||
        tile.tile_type == TileType::BendNW
}

// https://en.wikipedia.org/wiki/Shoelace_formula
fn calculate_area(tiles: &[Tile]) -> f64 {
    let mut sum = 0.0;

    for i in 0..tiles.len() {
        let current_tile = &tiles[i];
        let next_tile = &tiles[(i + 1) % tiles.len()];

        sum += current_tile.pos.0 as f64 * next_tile.pos.1 as f64;
        sum -= current_tile.pos.1 as f64 * next_tile.pos.0 as f64;
    }

    (sum.abs()) / 2.0
}

// https://en.wikipedia.org/wiki/Pick%27s_theorem
fn get_num_enclosed_points(path: &[Tile]) -> f64 {
    // area = i + (b/2) - 1
    let area = calculate_area(path);
    let num_points_on_path = path.len() as f64;

    area - num_points_on_path / 2.0 + 1.0
}

pub fn solve() {
    // ..F7.
    // .FJ|.
    // SJ.L7
    let file = File::open("src/day10/pipes.txt").expect("💣");
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<Tile>> = Vec::new();
    let mut pos = (0, 0);
    let mut start_tile = Tile {
        tile_type: TileType::Start,
        pos: (0, 0),
    };

    for line in reader.lines() {
        let line = line.expect("Nuuuu! 💣");
        if line.trim().is_empty() {
            break;
        }

        let row: Vec<Tile> = line.chars().enumerate().map(|(index, char)| {
            let current_pos = (index as i32, pos.1);
            let tile = match char {
                '.' => create_tile(TileType::Ground, current_pos),
                '|' => create_tile(TileType::Vertical, current_pos),
                '-' => create_tile(TileType::Horizontal, current_pos),
                'L' => create_tile(TileType::BendNE, current_pos),
                'J' => create_tile(TileType::BendNW, current_pos),
                '7' => create_tile(TileType::BendSW, current_pos),
                'F' => create_tile(TileType::BendSE, current_pos),
                'S' => create_tile(TileType::Start, current_pos),
                _ => panic!("Unknown tile: {}", char)
            };
            if tile.tile_type == TileType::Start {
                start_tile = tile.clone();
            }
            tile
        }).collect();
        map.push(row);
        pos.1 += 1;
    }

    let mut traversed_tiles: Vec<Tile> = Vec::new();
    let start_tile_type = get_tile_type(&map, &start_tile);
    let revealed_start_tile = Tile {
        tile_type: start_tile_type.clone(),
        pos: start_tile.pos,
    };

    count_distance(&map, &revealed_start_tile, &revealed_start_tile, 1, &mut traversed_tiles);

    let num_enclosed = get_num_enclosed_points(&traversed_tiles);

    println!("Sum: {}", num_enclosed);
}
