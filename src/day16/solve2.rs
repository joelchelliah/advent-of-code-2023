use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Clone)]
struct BeamState {
    direction: Direction,
    pos: (isize, isize),
}

fn follow_beam(beam: BeamState, contraption: &Vec<Vec<char>>, beam_history: Vec<BeamState>) -> Vec<BeamState> {
    let (x, y) = beam.pos;
    let out_of_bounds = x < 0 || x > contraption[0].len() as isize - 1 || y < 0 || y > contraption.len() as isize - 1;
    let loop_detected = beam_history.iter().any(|b| b.clone() == beam);

    if out_of_bounds || loop_detected {
        return beam_history.clone();
    }

    let mut new_beam_history = beam_history.clone();
    new_beam_history.push(beam.clone());

    let tile = contraption[y as usize][x as usize];

    let is_empty_tile =
        tile == '.' ||
        tile == '|' && (beam.direction == Direction::Up || beam.direction == Direction::Down) ||
        tile == '-' && (beam.direction == Direction::Left || beam.direction == Direction::Right);
    let is_split_tile =
        tile == '|' && (beam.direction == Direction::Left || beam.direction == Direction::Right) ||
        tile == '-' && (beam.direction == Direction::Up || beam.direction == Direction::Down);

    if is_empty_tile {
        let next_pos = match beam.direction {
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
        };

        follow_beam(BeamState { direction: beam.direction, pos: next_pos }, contraption, new_beam_history)
    } else if is_split_tile {
        let next_directions = match beam.direction {
            Direction::Left => [Direction::Up, Direction::Down],
            Direction::Right => [Direction::Up, Direction::Down],
            Direction::Up => [Direction::Left, Direction::Right],
            Direction::Down => [Direction::Left, Direction::Right],
        };
        let next_poses = match beam.direction {
            Direction::Left => [(x, y - 1), (x, y + 1)],
            Direction::Right => [(x, y - 1), (x, y + 1)],
            Direction::Up => [(x - 1, y), (x + 1, y)],
            Direction::Down => [(x - 1, y), (x + 1, y)],
        };
        let energized_tiles_from_first_split = follow_beam(
            BeamState { direction: next_directions[0], pos: next_poses[0] },
            contraption,
            new_beam_history
        );

        follow_beam(BeamState { direction: next_directions[1], pos: next_poses[1] }, contraption, energized_tiles_from_first_split)
    } else {
        let next_direction = match tile {
            '/' => match beam.direction {
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
            },
            '\\' => match beam.direction {
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
            },
            _ => panic!("💣")
        };
        let next_pos = match next_direction {
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
        };

        follow_beam(BeamState { direction: next_direction, pos: next_pos }, contraption, new_beam_history)
    }
}

pub fn solve() {
    // .|...\....
    // |.-.\.....
    // .....|-...
    let file = File::open("src/day16/contraption.txt").expect("💣");
    let reader = BufReader::new(file);

    let contraption: Vec<Vec<char>> = reader.lines().map(|line| line.unwrap().chars().collect()).collect();
    let num_rows = contraption.len();
    let num_cols = contraption[0].len();
    let beams: Vec<BeamState> = [
        (0..num_cols).map(|x| BeamState { direction: Direction::Down, pos: (x as isize, 0) }).collect::<Vec<BeamState>>(),
        (0..num_cols).map(|x| BeamState { direction: Direction::Up, pos: (x as isize, num_cols as isize) }).collect::<Vec<BeamState>>(),
        (0..num_rows).map(|y| BeamState { direction: Direction::Right, pos: (0, y as isize) }).collect::<Vec<BeamState>>(),
        (0..num_rows).map(|y| BeamState { direction: Direction::Left, pos: (num_rows as isize, y as isize) }).collect::<Vec<BeamState>>(),
        ].concat();

    let mut num_most_energized_tiles = 0;
    for beam in beams.iter() {
        let mut energized_tiles = follow_beam(beam.clone(), &contraption, Vec::new()).to_vec().iter().map(|b| b.pos).collect::<Vec<(isize, isize)>>();
        energized_tiles.sort();
        energized_tiles.dedup();

        if energized_tiles.len() > num_most_energized_tiles {
            num_most_energized_tiles = energized_tiles.len();
        }
    }

    println!("Sum: {}", num_most_energized_tiles);
}
