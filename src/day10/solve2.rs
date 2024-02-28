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

fn count_corner_tiles(tiles: &[Tile]) -> (i32, i32, i32, i32) {
    let mut ne_tiles = 0;
    let mut nw_tiles = 0;
    let mut sw_tiles = 0;
    let mut se_tiles = 0;

    for tile in tiles {
        match tile.tile_type {
            TileType::BendNE => ne_tiles += 1,
            TileType::BendNW => nw_tiles += 1,
            TileType::BendSW => sw_tiles += 1,
            TileType::BendSE => se_tiles += 1,
            _ => {}
        }
    }

    (ne_tiles, nw_tiles, sw_tiles, se_tiles)
}

fn count_relevant_tiles(tiles: &Vec<Tile>, straight_tile_type: TileType) -> i32 {
    let (mut ne_tiles, mut nw_tiles, mut sw_tiles, mut se_tiles) = count_corner_tiles(tiles);

    if straight_tile_type == TileType::Vertical {
        if ne_tiles > nw_tiles {
            ne_tiles -= nw_tiles;
            nw_tiles = 0;
        } else {
            nw_tiles -= ne_tiles;
            ne_tiles = 0;
        }

        if sw_tiles > se_tiles {
            sw_tiles -= se_tiles;
            se_tiles = 0;
        } else {
            se_tiles -= sw_tiles;
            sw_tiles = 0;
        }
    } else {
        if ne_tiles > se_tiles {
            ne_tiles -= se_tiles;
            se_tiles = 0;
        } else {
            se_tiles -= ne_tiles;
            ne_tiles = 0;
        }

        if nw_tiles > sw_tiles {
            nw_tiles -= sw_tiles;
            sw_tiles = 0;
        } else {
            sw_tiles -= nw_tiles;
            nw_tiles = 0;
        }
    }

    let combined_ne_sw = ne_tiles.max(sw_tiles);
    let combined_nw_se = nw_tiles.max(se_tiles);

    tiles.iter().filter(|tile| {
        tile.tile_type == straight_tile_type
    }).count() as i32 + combined_ne_sw + combined_nw_se
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

    let mut traversed_tiles: Vec<Tile> = Vec::new();
    let start_tile_type = get_tile_type(&map, &start_tile);
    let revealed_start_tile = Tile {
        tile_type: start_tile_type.clone(),
        pos: start_tile.pos,
    };

    count_distance(&map, &revealed_start_tile, &revealed_start_tile, 1, &mut traversed_tiles);

    let mut num_enclosed = 0;
    let is_odd = |&x: &i32| x % 2 != 0;


    for row in map {
        for tile in row {
            if traversed_tiles.contains(&tile) {
                continue;
            }

            let mut left_traversed_tiles: Vec<Tile> = Vec::new();
            let mut right_traversed_tiles: Vec<Tile> = Vec::new();
            let mut top_traversed_tiles: Vec<Tile> = Vec::new();
            let mut bottom_traversed_tiles: Vec<Tile> = Vec::new();

            for traversed_tile in &traversed_tiles {
                if traversed_tile.pos.1 == tile.pos.1 && traversed_tile.tile_type != TileType::Horizontal {
                    if traversed_tile.pos.0 < tile.pos.0 {
                        left_traversed_tiles.push(traversed_tile.clone());
                    } else if traversed_tile.pos.0 > tile.pos.0 {
                        right_traversed_tiles.push(traversed_tile.clone());
                    }
                }

                if traversed_tile.pos.0 == tile.pos.0 && traversed_tile.tile_type != TileType::Vertical {
                    if traversed_tile.pos.1 < tile.pos.1 {
                        top_traversed_tiles.push(traversed_tile.clone());
                    } else if traversed_tile.pos.1 > tile.pos.1 {
                        bottom_traversed_tiles.push(traversed_tile.clone());
                    }
                }
            }

            let num_relevant_left_tiles = count_relevant_tiles(&left_traversed_tiles, TileType::Vertical);
            let num_relevant_right_tiles = count_relevant_tiles(&right_traversed_tiles, TileType::Vertical);
            let num_relevant_top_tiles = count_relevant_tiles(&top_traversed_tiles, TileType::Horizontal);
            let num_relevant_bottom_tiles = count_relevant_tiles(&bottom_traversed_tiles, TileType::Horizontal);

            if is_odd(&num_relevant_left_tiles) &&
               is_odd(&num_relevant_right_tiles) &&
               is_odd(&num_relevant_top_tiles) &&
               is_odd(&num_relevant_bottom_tiles) {
                num_enclosed += 1;
            }
        }
    }

    println!("Sum: {}", num_enclosed);
}
