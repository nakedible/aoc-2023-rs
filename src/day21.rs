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
    n * (2*n - 2) + 1
} 

#[test_case("inputs/input-21.txt", 327 => matches Ok(97607))]
#[test_case("inputs/input-21.txt", 458 => matches Ok(191134))]
#[test_case("inputs/input-21.txt", 589 => matches Ok(315795))]
#[test_case("inputs/input-21.txt", 26501365 => matches Ok(637087163925555))]
pub fn puzzle2(filename: &str, steps: usize) -> Result<i64> {
    let (start, map) = parse_input(filename)?;
    let init = map.rows * 2;
    let (steps, rep) = if steps > init {
        ((steps % map.rows) + init, (steps - init) / map.rows)
    } else {
        (steps, 0)
    };
    let mut nodes = HashSet::new();
    nodes.insert((start.0 as isize, start.1 as isize));
    // let mut found = HashMap::new();
    for _i in 0..steps {
        nodes = nodes
            .iter()
            .flat_map(|&p| neighbours(&map, p))
            .collect::<HashSet<_>>();
        // let mut counts = HashMap::new();
        // nodes
        //     .iter()
        //     .map(|&p| (p.0.div_euclid(map.rows as isize), p.1.div_euclid(map.columns as isize)))
        //     .for_each(|p| {
        //         counts.entry(p).and_modify(|e| *e += 1).or_insert(1);
        //     });
        // for (pos, count) in counts.iter() {
        //     if *count == 7759 {
        //         found.entry(*pos).or_insert(i);
        //     }
        // }
        //println!("{}: {:?}", i, counts);
    }
    // println!("{:?}", found);

    let ret = if rep > 0 {
        // example:
        //    kal 
        //   kbihl
        //  kbiiihl
        // kbiiiiihl
        // ciiijiiig
        // ndiiiiifm
        //  ndiiifm
        //   ndifm
        //    nem
        // |      |      |      |      |      |      |      |
        // |      |      |  996 | 5858 |  994 |      |      |
        // |      |  996 | 6823 | 7808 | 6794 |  994 |      |
        // |      | 5871 | 7808 | 7759 | 7808 | 5843 |      |
        // |      |  989 | 6807 | 7808 | 6808 |  999 |      |
        // |      |      |  989 | 5856 |  999 |      |      |
        // |      |      |      |      |      |      |      |=
        let mut counts = HashMap::new();
        nodes
            .iter()
            .map(|&p| (p.0.div_euclid(map.rows as isize), p.1.div_euclid(map.columns as isize)))
            .for_each(|p| {
                counts.entry(p).and_modify(|e| *e += 1).or_insert(1usize);
            });
        // for y in -3..=3 {
        //     print!("|");
        //     for x in -3..=3 {
        //         if let Some(count) = counts.get(&(y, x)) {
        //             print!(" {:4} |", count);
        //         } else {
        //             print!("      |");
        //         }
        //     }
        //     println!();
        // }
        let b = counts[&(-1, -1)];
        let h = counts[&(-1, 1)];
        let d = counts[&(1, -1)];
        let f = counts[&(1, 1)];
        let i = counts[&(0, 0)];
        let i2 = counts[&(0, 1)];
        let k = counts[&(-2, -1)];
        let l = counts[&(-2, 1)];
        let m = counts[&(2, -1)];
        let n = counts[&(2, 1)];
        // println!("b: {}", b); // 6823
        // println!("h: {}", h); // 6794
        // println!("d: {}", d); // 6807
        // println!("f: {}", f); // 6808
        // println!("i: {}", i); // 7759
        // println!("i2: {}", i2); // 7808
        // println!("k: {}", k); // 996
        // println!("l: {}", l); // 994
        // println!("m: {}", m); // 989
        // println!("n: {}", n); // 999
        let base_count = nodes.len();
        let edges = rep * (b + h + d + f + k + l + m + n);
        let inner_count = calc_diamond_area(rep + 2) - calc_diamond_area(2);
        let inner = (inner_count / 2) * (i + i2) + rep * (i2 - i);
        (base_count + edges + inner) as i64
    } else {
        nodes.len() as i64
    };
    Ok(ret)
}
