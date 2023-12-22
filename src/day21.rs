use anyhow::Result;
use pathfinding::matrix::{directions, Matrix};
use std::collections::{HashMap, HashSet};
use test_case::test_case;

fn parse_input(filename: &str) -> Result<((usize, usize), Matrix<bool>)> {
    let input = std::fs::read_to_string(filename)?;
    let start = std::cell::Cell::new(None);
    let ret = Matrix::from_rows(input.lines().filter(|l| !l.is_empty()).enumerate().map(
        |(y, l)| {
            let start = &start;
            l.chars().enumerate().map(move |(x, c)| match c {
                '.' => false,
                '#' => true,
                'S' => {
                    start.set(Some((y, x)));
                    false
                }
                _ => unreachable!(),
            })
        },
    ))?;
    let start = start.take().expect("start should always be there");
    return Ok((start, ret));
}

fn clamp_pos(map: &Matrix<bool>, pos: (isize, isize)) -> (usize, usize) {
    (
        pos.0.rem_euclid(map.rows as isize) as usize,
        pos.1.rem_euclid(map.columns as isize) as usize,
    )
}

fn neighbours(
    map: &Matrix<bool>,
    pos: (isize, isize),
) -> impl IntoIterator<Item = (isize, isize)> + '_ {
    directions::DIRECTIONS_4
        .iter()
        .map(move |dir| (pos.0 + dir.0, pos.1 + dir.1))
        .filter(|&p| !map[clamp_pos(map, p)])
}

#[test_case("inputs/example-21-1.txt", 6 => matches Ok(16))]
#[test_case("inputs/example-21-1.txt", 10 => matches Ok(50))]
#[test_case("inputs/example-21-1.txt", 50 => matches Ok(1594))]
#[test_case("inputs/example-21-1.txt", 100 => matches Ok(6536))]
#[test_case("inputs/input-21.txt", 64 => matches Ok(3830))]
pub fn puzzle1(filename: &str, steps: usize) -> Result<i64> {
    let (start, map) = parse_input(filename)?;
    let mut nodes = HashSet::new();
    nodes.insert((start.0 as isize, start.1 as isize));
    for _ in 0..steps {
        nodes = nodes
            .iter()
            .flat_map(|&p| neighbours(&map, p))
            .collect::<HashSet<_>>();
    }
    let ret = nodes.len() as i64;
    Ok(ret)
}

#[test_case(1 => 1)]
#[test_case(2 => 5)]
#[test_case(3 => 13)]
#[test_case(4 => 25)]
#[test_case(5 => 41)]
fn calc_diamond_area(n: usize) -> usize {
    if n == 0 {
        0
    } else {
        n * (2*n - 2) + 1
    }
} 

#[test_case("inputs/example-21-1.txt", 6 => matches Ok(16))]
#[test_case("inputs/example-21-1.txt", 10 => matches Ok(50))]
#[test_case("inputs/example-21-1.txt", 50 => matches Ok(1594))]
#[test_case("inputs/example-21-1.txt", 100 => matches Ok(6536))]
#[test_case("inputs/example-21-1.txt", 500 => matches Ok(167004))]
#[test_case("inputs/example-21-1.txt", 1000 => matches Ok(668697))]
#[test_case("inputs/example-21-1.txt", 5000 => matches Ok(16733044))]
#[test_case("inputs/input-21.txt", 327 => matches Ok(97607))]
#[test_case("inputs/input-21.txt", 458 => matches Ok(191134))]
#[test_case("inputs/input-21.txt", 589 => matches Ok(315795))]
#[test_case("inputs/input-21.txt", 26501365 => matches Ok(637087163925555))]
pub fn puzzle2(filename: &str, steps: usize) -> Result<i64> {
    let (start, map) = parse_input(filename)?;
    let (rem, rep) = (steps % map.rows, steps / map.rows);
    let mut nodes = HashSet::new();
    nodes.insert((start.0 as isize, start.1 as isize));
    let mut prev_core = 0;
    let mut prev_adj = 0;
    let mut diff = 0;
    let mut spent_rep = 0;
    for _ in 0..rem {
        nodes = nodes
            .iter()
            .flat_map(|&p| neighbours(&map, p))
            .collect::<HashSet<_>>();
    }
    for i in 1..=rep {
        for _ in 0..map.rows {
            nodes = nodes
                .iter()
                .flat_map(|&p| neighbours(&map, p))
                .collect::<HashSet<_>>();
        }
        let total = nodes.len();
        let mut counts = HashMap::new();
        nodes
            .iter()
            .map(|&p| (p.0.div_euclid(map.rows as isize), p.1.div_euclid(map.columns as isize)))
            .for_each(|p| {
                counts.entry(p).and_modify(|e| *e += 1).or_insert(1usize);
            });
        let core = counts[&(0, 0)] + counts.get(&(0, 1)).unwrap_or(&0);
        let adj = total - core * ((calc_diamond_area(i - 1) - 1) / 2);
        spent_rep = i;
        if diff == adj - prev_adj {
            println!("found rep {} spent_rep {} core {} and diff {}", rep, spent_rep, core, diff);
            break;
        } else {
            diff = adj - prev_adj;
            prev_core = core;
            prev_adj = adj;
        }
    }
    let total = nodes.len();
    let edges = diff * (rep - spent_rep);
    let inner = prev_core * ((calc_diamond_area(rep-1) - calc_diamond_area(spent_rep-1)) / 2);
    let ret = total + edges + inner;
    Ok(ret as i64)
}
