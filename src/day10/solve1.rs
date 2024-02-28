use crate::util::read_lines;

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

fn get_next_valid_tile(map: &Vec<Vec<Tile>>, start_tile: &Tile) -> Tile {
            if start_tile.pos.0 - 1 >= 0 {
                let left_tile = get_tile_at(&map, (start_tile.pos.0 - 1, start_tile.pos.1));

                if is_valid_left_tile(&left_tile) {
                    return left_tile.clone();
                }
            }
            if start_tile.pos.0 + 1 < map[0].len() as i32 {
                let right_tile = get_tile_at(&map, (start_tile.pos.0 + 1, start_tile.pos.1));

                if is_valid_right_tile(&right_tile) {
                    return right_tile.clone();
                }
            }
            if start_tile.pos.1 - 1 >= 0 {
                let top_tile = get_tile_at(&map, (start_tile.pos.0, start_tile.pos.1 - 1));

                if is_valid_top_tile(&top_tile) {
                    return top_tile.clone();
                }
            }
            if start_tile.pos.1 + 1 < map.len() as i32 {
                let bottom_tile = get_tile_at(&map, (start_tile.pos.0, start_tile.pos.1 + 1));

                if is_valid_bottom_tile(&bottom_tile) {
                    return bottom_tile.clone();
                }
            }
            panic!("No valid tile found!");
}

fn count_distance(map: &Vec<Vec<Tile>>, tile: &Tile, prev_tile: &Tile, distance: i32) -> i32 {
    let next_pos = get_next_pos(&tile, prev_tile);
    let next_tile = get_tile_at(&map, next_pos);

    if next_tile.tile_type == TileType::Start {
        return (distance + 1) / 2;
    }

    count_distance(map, next_tile, tile, distance + 1)
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

pub fn solve() {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    let mut pos = (0, 0);
    let mut start_tile = Tile {
        tile_type: TileType::Start,
        pos: (0, 0),
    };

    for line in read_lines("src/day10/pipes.txt").unwrap() {
        let line = line.unwrap();
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

    let tile_after_start = get_next_valid_tile(&map, &start_tile);
    let longest_distance = count_distance(&map, &tile_after_start, &start_tile, 1);

    println!("Sum: {}", longest_distance);
}
