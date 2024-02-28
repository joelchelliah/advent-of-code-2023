use crate::util::read_lines;

struct HailStone {
    pos: (isize, isize, isize),
    velocity: (isize, isize, isize),
}

impl HailStone {
    fn intersects_at(&self, other: &HailStone, test_area: (f64, f64)) -> bool {
        let (x1, y1) = (self.pos.0 as f64, self.pos.1 as f64);
        let (vx1, vy1) = (self.velocity.0 as f64, self.velocity.1 as f64);
        let (x2, y2) = (other.pos.0 as f64, other.pos.1 as f64);
        let (vx2, vy2) = (other.velocity.0 as f64, other.velocity.1 as f64);

        let determinant = vx1 * vy2 - vy1 * vx2;
        if determinant.abs() < 1e-10 { return false; } // parallel

        let u = ((x2 - x1) * vy2 - (y2 - y1) * vx2) / determinant;
        let v = ((x2 - x1) * vy1 - (y2 - y1) * vx1) / determinant;
        if u < 0.0 || v < 0.0 { return false; } // past intersection

        let x = x1 + u * vx1;
        let y = y1 + u * vy1;
        let invalid = x < test_area.0 || x > test_area.1 || y < test_area.0 || y > test_area.1;

        return !invalid
    }
}

fn parse_coords(coords: &str) -> (isize, isize, isize) {
    let parts: Vec<isize> = coords.split(",").map(|x| x.trim().parse().unwrap()).collect();
    (parts[0], parts[1], parts[2])
}

pub fn solve() {
    let hailstones = read_lines("src/day24/hailstone.txt").unwrap().map(|line| {
            let line = line.unwrap();
            let mut parts = line.split("@").map(|x| x.trim());

            HailStone {
                pos: parse_coords(parts.next().unwrap()),
                velocity: parse_coords(parts.next().unwrap()),
            }
    }).collect::<Vec<HailStone>>();

    let test_area = (200000000000000.0, 400000000000000.0);
    let mut num_intersections = 0;

    for i in 0..hailstones.len() {
        for j in i+1..hailstones.len() {
            if (&hailstones[i]).intersects_at(&hailstones[j], test_area) {
                num_intersections += 1;
            }
        }
    }

    println!("Sum: {}", num_intersections);
}
