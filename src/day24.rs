use anyhow::Result;
use test_case::test_case;

type Vec3 = (i64, i64, i64);

fn parse_input(filename: &str) -> Result<Vec<(Vec3, Vec3)>> {
    let input = std::fs::read_to_string(filename)?;
    let ret = input.lines().map(|l| {
        let (pos, vel) = l.split_once(" @ ").unwrap();
        let (x, pos) = pos.split_once(", ").unwrap();
        let (y, z) = pos.split_once(", ").unwrap();
        let (vx, vel) = vel.split_once(", ").unwrap();
        let (vy, vz) = vel.split_once(", ").unwrap();
        let x = x.trim().parse::<i64>().unwrap();
        let y = y.trim().parse::<i64>().unwrap();
        let z = z.trim().parse::<i64>().unwrap();
        let vx = vx.trim().parse::<i64>().unwrap();
        let vy = vy.trim().parse::<i64>().unwrap();
        let vz = vz.trim().parse::<i64>().unwrap();
        ((x, y, z), (vx, vy, vz))
    }).collect();
    return Ok(ret);
}

fn vec3_add(a: Vec3, b: Vec3) -> Vec3 {
    (
        a.0 + b.0,
        a.1 + b.1,
        a.2 + b.2,
    )
}

fn vec3_sub(a: Vec3, b: Vec3) -> Vec3 {
    (
        a.0 - b.0,
        a.1 - b.1,
        a.2 - b.2,
    )
}

fn vec3_mulsca(a: Vec3, b: i64) -> Vec3 {
    (
        a.0 * b,
        a.1 * b,
        a.2 * b,
    )
}

fn calc_intersection_2d((ap, av): (Vec3, Vec3), (bp, bv): (Vec3, Vec3)) -> Option<(f64, f64)> {
    let det = av.0 * bv.1 - av.1 * bv.0;
    if det == 0 {
        return None;
    }
    let d = vec3_sub(bp, ap);
    let at = (d.0 * bv.1 - d.1 * bv.0) as f64 / det as f64;
    let bt = (d.0 * av.1 - d.1 * av.0) as f64 / det as f64;
    if at < 0.0 || bt < 0.0 {
        return None;
    }
    let rx = ap.0 as f64 + at * av.0 as f64;
    let ry = ap.1 as f64 + at * av.1 as f64;
    Some((rx, ry))
}

fn in_range_xy(pos: (f64, f64), min: f64, max: f64) -> bool {
    pos.0 >= min && pos.0 <= max && pos.1 >= min && pos.1 <= max
}

#[test_case("inputs/example-24-1.txt", 7.0, 27.0 => matches Ok(2))]
#[test_case("inputs/input-24.txt", 200000000000000.0, 400000000000000.0 => matches Ok(14672))]
pub fn puzzle1(filename: &str, min: f64, max: f64) -> Result<i64> {
    let input = parse_input(filename)?;
    let mut ret = 0;
    for i in 0..input.len() {
        for j in 0..i {
            if let Some(cross) = calc_intersection_2d(input[i], input[j]) {
                if in_range_xy(cross, min, max) {
                    ret += 1;
                }
            }
        }
    }
    Ok(ret)
}

fn d(m: Vec3, n: Vec3, o: Vec3, p: Vec3) -> i128 {
    (m.0 as i128 - n.0 as i128) * (o.0 as i128 - p.0 as i128) +
    (m.1 as i128 - n.1 as i128) * (o.1 as i128 - p.1 as i128) +
    (m.2 as i128 - n.2 as i128) * (o.2 as i128 - p.2 as i128)
}

fn exact_div(a: i128, b: i128) -> Option<i128> {
    if b == 0 || a % b != 0 {
        None
    } else {
        Some(a / b)
    }
}

fn mu_ab((ap, av): (Vec3, Vec3), (bp, bv): (Vec3, Vec3)) -> Option<(i64, i64)> {
    let (an, bn) = (vec3_add(ap, av), vec3_add(bp, bv));
    let mu_a = exact_div(
        d(ap, bp, bn, bp) * d(bn, bp, an, ap) - d(ap, bp, an, ap) * d(bn, bp, bn, bp),
        d(an, ap, an, ap) * d(bn, bp, bn, bp) - d(bn, bp, an, ap) * d(bn, bp, an, ap),
    )?;
    let mu_b = exact_div(
        d(ap, bp, bn, bp) + mu_a * d(bn, bp, an, ap),
        d(bn, bp, bn, bp)
    )?;
    Some((mu_a.try_into().unwrap(), mu_b.try_into().unwrap()))
}

fn inter_point((ap, av): (Vec3, Vec3), (bp, bv): (Vec3, Vec3)) -> Option<Vec3> {
    let (mu_a, mu_b) = mu_ab((ap, av), (bp, bv))?;
    let intera = vec3_add(ap, vec3_mulsca(av, mu_a));
    let interb = vec3_add(bp, vec3_mulsca(bv, mu_b));
    if intera == interb {
        Some(intera)
    } else {
        None
    }
}

#[test_case("inputs/example-24-1.txt" => matches Ok(47))]
#[test_case("inputs/input-24.txt" => matches Ok(646810057104753))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let input: [(Vec3, Vec3); 3] = input[..3].try_into().unwrap();
    let mut pos = (0, 0, 0);
    const MIN_VEL: i64 = -500;
    const MAX_VEL: i64 = 500;
    'top: for xv in MIN_VEL..=MAX_VEL {
        for yv in MIN_VEL..=MAX_VEL {
            for zv in MIN_VEL..=MAX_VEL {
                let cv = (xv as i64, yv as i64, zv as i64);
                let rel_input = input.map(|(p, v)| (p, vec3_sub(v, cv)));
                let Some(i1) = inter_point(rel_input[0], rel_input[1]) else {
                    continue;
                };
                let Some(i2) = inter_point(rel_input[0], rel_input[2]) else {
                    continue;
                };
                if i1 == i2 {
                    println!("{xv} {yv} {zv} {i1:?}");
                    pos = i1;
                    break 'top;
                }
            }
        }
    }
    let ret = pos.0 + pos.1 + pos.2;
    Ok(ret)
}
