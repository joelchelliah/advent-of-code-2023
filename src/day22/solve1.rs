use std::cmp::min;
use std::cmp::max;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::ops::RangeInclusive;

type Pos = (usize, usize, usize);

#[derive(Clone, PartialEq, Eq, Hash)]
struct Brick {
    from: Pos,
    to: Pos,
}

impl Brick {
    fn new(from: &str, to: &str) -> Self {
        let from: Vec<usize> = from.split(",").map(|n| n.parse().unwrap()).collect();
        let to: Vec<usize> = to.split(",").map(|n| n.parse().unwrap()).collect();

        Brick { from: (from[0], from[1], from[2]), to: (to[0], to[1], to[2]) }
    }

    fn intersects_or_invalid(&self, positions: &HashSet<Pos>) -> bool {
        let self_positions = self.get_positions();

        self_positions.iter().any(|(_,_,z)| *z == 0) ||
        self_positions.intersection(&positions).next().is_some()
    }

    fn fall(&self, locked_positions: &HashSet<Pos>) -> Brick {
        let brick = self.fall_once();

        if brick.intersects_or_invalid(locked_positions) {
            return self.clone();
        } else {
            return brick.fall(locked_positions);
        }
    }

    fn fall_once(&self) -> Brick {
        let mut brick = self.clone();

        brick.from.2 -= 1;
        brick.to.2 -= 1;
        brick
    }

    fn get_positions(&self) -> HashSet<Pos> {
        range(self.from.0, self.to.0)
            .flat_map(move |x| range(self.from.1, self.to.1)
                .flat_map(move |y| range(self.from.2, self.to.2)
                    .map(move |z| (x, y, z)))).collect()
    }

    fn supports(&self, brick: &Brick) -> bool {
        if brick == self ||
           min(brick.from.2, brick.to.2) <= max(self.from.2, self.to.2) {
            return false;
        }

        brick.fall_once().intersects_or_invalid(&self.get_positions())
    }
}

fn range(from: usize, to: usize) -> RangeInclusive<usize> {
    min(from, to)..=max(from, to)
}

pub fn solve() {
    let file = File::open("src/day22/snapshot.txt").expect("💣");
    let reader = BufReader::new(file);

    let mut bricks: Vec<Brick> = reader.lines().map(|line| {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split("~").collect();

        Brick::new(parts[0], parts[1])
    }).collect();

    bricks.sort_by(|a, b| {
        min(a.from.2, a.to.2).cmp(&min(b.from.2, b.to.2))
        .then(max(a.from.2, a.to.2).cmp(&max(b.from.2, b.to.2)))
    });

    let mut locked_positions: HashSet<Pos> = HashSet::new();
    let mut placed_bricks: Vec<Brick> = Vec::new();

    for brick in &bricks {
        let moved_brick = brick.clone().fall(&locked_positions);

        locked_positions.extend(moved_brick.get_positions());
        placed_bricks.push(moved_brick.clone());
    }


    let mut bricks_that_can_be_removed: HashSet<&Brick> = HashSet::new();
    let mut bricks_that_cannot_be_removed: HashSet<&Brick> = HashSet::new();

    for i in 0..placed_bricks.len() {
        let brick = &placed_bricks[i];
        let relying_bricks = placed_bricks.iter().filter(|rel| brick.supports(rel)).collect::<Vec<&Brick>>();

        if relying_bricks.len() == 0 {
            bricks_that_can_be_removed.insert(&brick);
        }

        let supporting_bricks = placed_bricks.iter().filter(|sup| sup.supports(brick)).collect::<Vec<&Brick>>();

        for supporting_brick in supporting_bricks {
            if bricks_that_cannot_be_removed.contains(&supporting_brick) {
                continue;
            }

            let mut locked_positions = locked_positions.clone();
            locked_positions = locked_positions
                .difference(&brick.get_positions()).cloned().collect::<HashSet<(usize, usize, usize)>>()
                .difference(&supporting_brick.get_positions()).cloned().collect();

            let moved_brick = brick.clone().fall(&locked_positions);
            if moved_brick == *brick {
                bricks_that_can_be_removed.insert(&supporting_brick);
            } else {
                bricks_that_cannot_be_removed.insert(&supporting_brick);
            }
        }
    }

    let num = bricks_that_can_be_removed.difference(&bricks_that_cannot_be_removed).cloned().count();

    println!("Sum: {}", num);
}
