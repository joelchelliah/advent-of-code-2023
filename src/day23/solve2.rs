use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::fs::File;

type Pos = (usize, usize);

fn to_pos(pos: (isize, isize)) -> Pos {
    (pos.0 as usize, pos.1 as usize)
}

fn to_i_pos(pos: Pos) -> (isize, isize) {
    (pos.0 as isize, pos.1 as isize)
}

fn get_neighbors_iter(pos: Pos) -> impl Iterator<Item = (isize, isize)> {
    let (x, y) = to_i_pos(pos);
    vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter()
}

fn is_valid_pos(map: &Vec<Vec<char>>, pos: (isize, isize)) -> bool {
    let (x, y) = pos;
    x >= 0 &&
    y >= 0 &&
    x < map[0].len() as isize &&
    y < map.len() as isize &&
    map[y as usize][x as usize] != '#'
}

fn get_junctions(map: &Vec<Vec<char>>, start: Pos, end: Pos) -> Vec<Pos> {
    let mut junctions = vec![start, end];

    for x in 0..map[0].len() {
        for y in 0..map.len() {
            let pos = (x, y);
            let c = map[y][x];

            if c == '#' { continue; }

            let num_neighbors = get_neighbors_iter(pos)
                .filter(|pos| is_valid_pos(map, *pos))
                .count();

            if num_neighbors > 2 { junctions.push(pos); }
        }
    }
    junctions
}

fn build_adjacency_list(map: &Vec<Vec<char>>, start: Pos, end: Pos,) -> Vec<Vec<Pos>> {
    let junctions = get_junctions(map, start, end);
    let mut adjacency_list: Vec<Vec<(usize, usize)>> = Vec::new();

    for (junction_i, &junction_pos) in junctions.iter().enumerate() {
        let mut connections = Vec::new();
        let neighbors = get_neighbors_iter(junction_pos)
            .filter(|pos| is_valid_pos(map, *pos))
            .map(|pos| to_pos(pos));

        let mut prev_neighbor_pos = junction_pos;
        for mut neighbor_pos in neighbors {
            let mut dist = 1;

            let connection_index = loop {
                if let Some(index) = junctions.iter().position(|c| *c == neighbor_pos) {
                    prev_neighbor_pos = junction_pos;
                    break index;
                }

                let next_neighbor_pos = get_neighbors_iter(neighbor_pos)
                    .filter(|pos|
                        to_pos(*pos) != prev_neighbor_pos &&
                        is_valid_pos(map, *pos)
                    ).collect::<Vec<(isize, isize)>>();

                assert_eq!(next_neighbor_pos.len(), 1);

                prev_neighbor_pos = neighbor_pos;
                neighbor_pos = to_pos(next_neighbor_pos[0]);
                dist += 1;
            };

            connections.push((connection_index, dist));
        }

        adjacency_list.insert(junction_i, connections);
    }

    adjacency_list
}

fn walk(adjacency_list: &Vec<Vec<(usize, usize)>>, start: usize, end: usize, mut visited: HashSet<usize>) -> usize {
    if start == end { return 0; }

    visited.insert(start);

    if let Some(dist) = adjacency_list[start].iter().find_map(|(i, distance)| (*i == end).then_some(*distance)) {
        return dist;
    }

    adjacency_list[start].iter().filter_map(|&(i, distance)| {
        if visited.contains(&i) { return None; }

        Some(distance + walk(adjacency_list, i, end, visited.clone()))
    }).max().unwrap_or(0)
}

pub fn solve() {
    let file = File::open("src/day23/hiking_trails.txt").expect("💣");
    let reader = BufReader::new(file);

    let map: Vec<Vec<char>> = reader.lines()
        .map(|line| line.unwrap().chars()
            .map(|c| c).collect()).collect();
    let start = (1, 0);
    let end = (map[0].len() - 2, map.len() - 1);
    let adjacency_list = build_adjacency_list(&map, start, end);
    let count = walk(&adjacency_list, 0, 1, HashSet::new());

    println!("Sum: {}", count);
}
