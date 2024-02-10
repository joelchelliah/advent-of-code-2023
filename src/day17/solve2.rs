use std::collections::BinaryHeap;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::cmp::Ordering;
use std::fs::File;

#[derive(Eq, PartialEq, Clone, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
use Direction::*;

impl Direction {
    fn cw(&self) -> Direction {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn ccw(&self) -> Direction {
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    fn get_pos(&self, (x,y): (isize, isize)) -> (isize, isize) {
        match self {
            Up => (x, y - 1),
            Right => (x + 1, y),
            Down => (x, y + 1),
            Left => (x - 1, y),
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
struct Node {
    pos: (isize, isize),
    loss: usize,
    direction: Direction,
    num_steps: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.loss.cmp(&self.loss)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.loss.cmp(&self.loss))
    }
}

fn valid_pos_or_none((x, y): (isize, isize), map: &Vec<Vec<usize>>) -> Option<(isize, isize)> {
    if x >= 0 && x < map[0].len() as isize && y >= 0 && y < map.len() as isize {
        return Some((x,y))
    } else {
        return None
    }
}


pub fn solve() {
    // 2413432311323
    // 3215453535623
    // 3255245654254
    let file = File::open("src/day17/map.txt").expect("💣");
    let reader = BufReader::new(file);

    let loss_map: Vec<Vec<usize>> = reader.lines().map(|line| line.unwrap().chars().map(|c| c.to_digit(10).unwrap() as usize).collect()).collect();

    let min_steps = 4;
    let max_steps = 10;
    let target_pos = (loss_map[0].len() as isize - 1, loss_map.len() as isize - 1);

    let mut visited: HashSet<((isize, isize), Direction, usize)> = HashSet::new();
    let mut queue: BinaryHeap<Node> = BinaryHeap::new();

    queue.push(Node { pos: (0, 0), loss: 0, direction: Right, num_steps: 0 });
    queue.push(Node { pos: (0, 0), loss: 0, direction: Down, num_steps: 0 });

    while !queue.is_empty() {
        let node = queue.pop().unwrap();
        let (loss, pos, direction, num_steps) = (node.loss, node.pos, node.direction, node.num_steps);

        if pos == target_pos && num_steps >= min_steps {
            println!("Total loss: {}\n", loss);
            break;
        }

        if !visited.insert((pos, direction.clone(), num_steps)) { continue };

        for new_dir in [
            direction.cw(),
            direction.ccw(),
            direction.clone()
        ].iter() {
            let new_pos = new_dir.get_pos(pos);
            if let Some((x, y)) = valid_pos_or_none(new_pos, &loss_map) {
                let new_loss = loss + loss_map[y as usize][x as usize];

                if new_dir == &direction && num_steps < max_steps {
                    queue.push(Node { pos: new_pos, loss: new_loss, direction: new_dir.clone(), num_steps: num_steps + 1});
                } else if new_dir != &direction && num_steps >= min_steps {
                    queue.push(Node { pos: new_pos, loss: new_loss, direction: new_dir.clone(), num_steps: 1});
                }
            };
        }
    }
}
