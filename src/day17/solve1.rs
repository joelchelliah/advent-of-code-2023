use std::collections::BinaryHeap;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::cmp::Ordering;
use std::iter::once;
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

    let start_node = Node { pos: (0, 0), loss: 0, direction: Up, num_steps: 0 };
    let target_pos = (loss_map[0].len() as isize - 1, loss_map.len() as isize - 1);

    let mut queue: BinaryHeap<Node> = once(start_node).collect();
    let mut visited: HashSet<((isize, isize), Direction, usize)> = HashSet::new();

    while !queue.is_empty() {
        let node = queue.pop().unwrap();
        let (loss, pos, direction, num_steps) = (node.loss, node.pos, node.direction, node.num_steps);

        if pos == target_pos {
            println!("Total loss: {}\n", loss);
            break;
        }

        if !visited.insert((pos, direction.clone(), num_steps)) { continue };

        for dir in [
            direction.cw(),
            direction.ccw(),
            direction.clone()
        ].iter() {
            let pos = dir.get_pos(pos);
            if let Some((x, y)) = valid_pos_or_none(pos, &loss_map) {
                let new_loss = loss + loss_map[y as usize][x as usize];

                if dir == &direction {
                    if num_steps < 3 {
                        queue.push(Node { pos: (x, y), loss: new_loss, direction: dir.clone(), num_steps: num_steps + 1});
                    }
                } else {
                    queue.push(Node { pos: (x, y), loss: new_loss, direction: dir.clone(), num_steps: 1});
                }
            };
        }
    }
}
