use anyhow::Result;
use test_case::test_case;

type Pos = (f64, f64, f64);
type Vel = (f64, f64, f64);

fn parse_input(filename: &str) -> Result<Vec<(Pos, Vel)>> {
    let input = std::fs::read_to_string(filename)?;
    let ret = input.lines().map(|l| {
        let (pos, vel) = l.split_once(" @ ").unwrap();
        let (x, pos) = pos.split_once(", ").unwrap();
        let (y, z) = pos.split_once(", ").unwrap();
        let (vx, vel) = vel.split_once(", ").unwrap();
        let (vy, vz) = vel.split_once(", ").unwrap();
        let x = x.trim().parse::<f64>().unwrap();
        let y = y.trim().parse::<f64>().unwrap();
        let z = z.trim().parse::<f64>().unwrap();
        let vx = vx.trim().parse::<f64>().unwrap();
        let vy = vy.trim().parse::<f64>().unwrap();
        let vz = vz.trim().parse::<f64>().unwrap();
        ((x, y, z), (vx, vy, vz))
    }).collect();
    return Ok(ret);
}

fn calc_intersection((ap, av): (Pos, Vel), (bp, bv): (Pos, Vel)) -> Option<Pos> {
    let det = av.0 * bv.1 - av.1 * bv.0;
    if det == 0.0 {
        return None;
    }
    let dx = bp.0 - ap.0;
    let dy = bp.1 - ap.1;
    let at = (dx * bv.1 - dy * bv.0) / det;
    let bt = (dx * av.1 - dy * av.0) / det;
    if at < 0.0 || bt < 0.0 {
        return None;
    }
    let rx = ap.0 + at * av.0;
    let ry = ap.1 + at * av.1;
    let ret = (rx, ry, 0.0);
    Some(ret)
}

fn in_range(pos: Pos, min: f64, max: f64) -> bool {
    pos.0 >= min && pos.0 <= max && pos.1 >= min && pos.1 <= max
}

#[test_case("inputs/example-24-1.txt", 7.0, 27.0 => matches Ok(4361))]
#[test_case("inputs/input-24.txt", 200000000000000.0, 400000000000000.0 => matches Ok(557705))]
pub fn puzzle1(filename: &str, min: f64, max: f64) -> Result<i64> {
    let input = parse_input(filename)?;
    let mut ret = 0;
    for i in 0..input.len() {
        for j in 0..i {
            if let Some(cross) = calc_intersection(input[i], input[j]) {
                if in_range(cross, min, max) {
                    ret += 1;
                }
            }
        }
    }
    Ok(ret)
}

// #[test_case("inputs/example-24-1.txt" => matches Ok(467835))]
// #[test_case("inputs/input-24.txt" => matches Ok(84266818))]
// pub fn puzzle2(filename: &str, min: f64, max: f64) -> Result<i64> {
//     let input = parse_input(filename)?;

//     Ok(0)
// }
