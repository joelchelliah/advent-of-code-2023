use crate::util::read_lines;

#[derive(Clone, Debug)]
struct HailStone {
    pos: (f64, f64, f64),
    velocity: (f64, f64, f64),
}

fn parse_coords(coords: &str) -> (f64, f64, f64) {
    let parts: Vec<f64> = coords.split(",").map(|x| x.trim().parse().unwrap()).collect();
    (parts[0], parts[1], parts[2])
}

fn get_relative_velocity(a: &HailStone, b: &HailStone) -> (f64, f64, f64) {
    let vx = a.velocity.0 - b.velocity.0;
    let vz = a.velocity.2 - b.velocity.2;

    (vx, 0.0, vz)
}

fn get_relative_pos(a: &HailStone, b: &HailStone) -> (f64, f64, f64) {
    let x = a.pos.0 - b.pos.0;
    let y = a.pos.1 - b.pos.1;
    let z = a.pos.2 - b.pos.2;

    (x, y, z)
}

fn get_pos_at_time(hail: &HailStone, time: f64) -> (f64, f64, f64) {
    let x = hail.pos.0 + ( time * hail.velocity.0 );
    let y = hail.pos.1 + (time * hail.velocity.1);
    let z = hail.pos.2 + (time * hail.velocity.2);

    (x, y, z)
}

fn get_velocity(pos1: (f64, f64, f64), pos2: (f64, f64, f64), time: f64) -> (f64, f64, f64) {
    let vx = (pos2.0 - pos1.0) / time;
    let vy = (pos2.1 - pos1.1) / time;
    let vz = (pos2.2 - pos1.2) / time;

    (vx, vy, vz)
}

fn get_initial_pos(pos: (f64, f64, f64), velocity: (f64, f64, f64), time: f64) -> (f64, f64, f64) {
    let x = pos.0 - (velocity.0 * time);
    let y = pos.1 - (velocity.1 * time);
    let z = pos.2 - (velocity.2 * time);

    (x, y, z)
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


    // SOLUTION FROM: https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kicuapd/?utm_source=share&utm_medium=web2x&context=3
    // If standing on a hail we will see the rock travel in a straight line,
    // pass through us and two other points on two other pieces of hail that zip by
    // there must be two vectors from our hail to the other two collisions (v1 and v2)
    // such that v1 = m * v2 where m is some unknown scalar multiplier.
    // we can make v1 = v2 by dividing one of the x,y or z components by itself to ensure
    // it is equal to 1. Then solve.

    // Select three hail that have one of the same speed components (e.g. vy):
    let locked_vy = hailstones[0].velocity.1;
    let hails =
        hailstones.iter()
                  .filter(|stone| stone.velocity.1 == locked_vy)
                  .collect::<Vec<&HailStone>>();
    let [hail0, hail1, hail2] = hails[0..3] else { panic!("⛄") };

    // Relative velocities of hail 1 and 2 to hail 0
    let (h1_rel_vx, _, h1_rel_vz) = get_relative_velocity(hail1, hail0);
    let (h2_rel_vx, _, h2_rel_vz) = get_relative_velocity(hail2, hail0);

    // Relative initial pos of hail 1 and 2 to hail 0
    let (h1_rel_x, h1_rel_y, h1_rel_z) = get_relative_pos(hail1, hail0);
    let (h2_rel_x, h2_rel_y, h2_rel_z) = get_relative_pos(hail2, hail0);

    // 1st Hail equations
    // x = h1_rel_x + h1_rel_vx * t1
    // y = h1_rel_y
    // z = h1_rel_z + h1_rel_vz * t1

    // 2nd Hail equations
    // x = h2_rel_x + h2_rel_vx * t2
    // y = h2_rel_y
    // z = h2_rel_z + h2_rel_vz * t2

    // Divide all equations by the y component to make y = 1 and ensure both vectors are the same

    // 1st Set results
    // x = (h1_rel_x + h1_rel_vx * t1) / h1_rel_y
    // y = 1
    // z = (h1_rel_z + h1_rel_vz * t1) / h1_rel_y

    // 2nd Set results
    // x = (h2_rel_x + h2_rel_vx * t2) / h2_rel_y
    // y = 1
    // z = (h2_rel_z + h2_rel_vz * t2) / h2_rel_y

    // Solve set of two linear equations x=x and z=z
    let num = (h2_rel_y * h1_rel_x * h1_rel_vz) -
                   (h1_rel_vx * h2_rel_y * h1_rel_z) +
                   (h1_rel_y * h2_rel_z * h1_rel_vx) -
                   (h1_rel_y * h2_rel_x * h1_rel_vz);
    let den = h1_rel_y * ((h1_rel_vz * h2_rel_vx) - (h1_rel_vx * h2_rel_vz));
    let t2 = num / den;

    // Substitute t2 into a t1 equation
    let num = (h1_rel_y * h2_rel_x) +
                   (h1_rel_y * h2_rel_vx * t2) -
                   (h2_rel_y * h1_rel_x);
    let den = h2_rel_y * h1_rel_vx;
    let t1 = num / den;

    let collision1 = get_pos_at_time(hail1, t1);
    let collision2 = get_pos_at_time(hail2, t2);

    let rock_velocity = get_velocity(collision1, collision2, t2 - t1);

    let (x, y, z) = get_initial_pos(collision1, rock_velocity, t1);

    println!("Sum: {}", (x + y + z).ceil());
}
