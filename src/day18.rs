use anyhow::Result;
use pathfinding::matrix::{directions, Matrix};
use test_case::test_case;

type Instr = ((isize, isize), usize, u32);

fn parse_input(filename: &str) -> Result<Vec<Instr>> {
    let input = std::fs::read_to_string(filename)?;
    let ret = input.lines().map(|l| {
        let (dir, rest) = l.split_once(' ').unwrap();
        let (count, color) = rest.split_once(' ').unwrap();
        let dir = match dir {
            "U" => directions::N,
            "D" => directions::S,
            "L" => directions::W,
            "R" => directions::E,
            _ => unreachable!()
        };
        let count = count.parse().unwrap();
        let color = u32::from_str_radix(&color[2..color.len()-1], 16).unwrap();
        (dir, count, color)
    }).collect();
    return Ok(ret);
}

#[test_case("inputs/example-18-1.txt" => matches Ok(62))]
#[test_case("inputs/input-18.txt" => matches Ok(67891))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    // let mut map = Matrix::new(1000, 1000, 0xffffffu32);
    // let mut pos = (500, 500);
    // for (dir, count, color) in input {
    //     for _ in 0..count {
    //         pos = map.move_in_direction(pos, dir).unwrap();
    //         map[pos] = color;
    //     }
    // }
    // let outer = map.bfs_reachable((0, 0), false, |pos| {
    //     map[pos] == 0xffffffu32
    // });
    // for o in outer {
    //     map[o] = 0u32;
    // }
    // let ret = map.values().filter(|v| **v != 0u32).count() as i64;
    let mut pos = (0, 0);
    let mut lines = vec![(0, 0)];
    let mut border = 0;
    for (dir, count, _) in &input {
        border += *count as i64;
        pos.0 = pos.0 + dir.0 * *count as isize;
        pos.1 = pos.1 + dir.1 * *count as isize;
        lines.push((pos.0 as i64, pos.1 as i64));
    }
    let ret = calc_area(lines) + border / 2 + 1;
    Ok(ret)
}

fn fix_input(input: &Vec<Instr>) -> Vec<Instr> {
    input.iter().map(|(_, _, color)| {
        let dir = match color & 0xf {
            0 => directions::E,
            1 => directions::S,
            2 => directions::W,
            3 => directions::N,
            _ => unreachable!(),
        };
        let count = (color >> 4) as usize;
        (dir, count, *color)
    }).collect()
}

fn calc_area(polygon: Vec<(i64, i64)>) -> i64 {
    let ret = polygon.windows(2).map(|w| {
        w[0].0 * w[1].1 - w[1].0 * w[0].1
    }).sum::<i64>();
    ret.abs() / 2
}

#[test_case("inputs/example-18-1.txt" => matches Ok(952408144115))]
#[test_case("inputs/input-18.txt" => matches Ok(94116351948493))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let input = fix_input(&input);
    let mut pos = (0, 0);
    let mut lines = vec![(0, 0)];
    let mut border = 0;
    for (dir, count, _) in &input {
        border += *count as i64;
        pos.0 = pos.0 + dir.0 * *count as isize;
        pos.1 = pos.1 + dir.1 * *count as isize;
        lines.push((pos.0 as i64, pos.1 as i64));
    }
    let ret = calc_area(lines) + border / 2 + 1;
    Ok(ret as i64)
}
