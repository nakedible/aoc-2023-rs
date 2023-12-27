use anyhow::Result;
use test_case::test_case;

fn parse_input(filename: &str) -> Result<Vec<(i64, i64)>> {
    let input = std::fs::read_to_string(filename)?;
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap());
    let distances = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap());
    let ret = times.zip(distances).collect::<Vec<_>>();
    Ok(ret)
}

fn find_roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    let d = b * b - 4.0 * a * c;
    assert!(d > 0.0);
    let x1 = (-b + d.sqrt()) / (2.0 * a);
    let x2 = (-b - d.sqrt()) / (2.0 * a);
    (x1, x2)
}

fn next_int(a: f64) -> i64 {
    if a.fract() == 0.0 {
        a as i64 + 1
    } else {
        a.ceil() as i64
    }
}

fn prev_int(a: f64) -> i64 {
    if a.fract() == 0.0 {
        a as i64 - 1
    } else {
        a.floor() as i64
    }
}

#[test_case("inputs/example-06-1.txt" => matches Ok(288))]
#[test_case("inputs/input-06.txt" => matches Ok(1710720))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let mut ret = 1;
    for (time, distance) in input {
        // let mut ways = 0;
        // for dur in 0..time {
        //     let dist = (time - dur) * dur;
        //     if dist > distance {
        //         ways += 1;
        //     }
        // }
        let (x1, x2) = find_roots(-1.0, time as f64, -1.0 * distance as f64);
        let ways = prev_int(x2) - next_int(x1) + 1;
        ret *= ways;
    }
    Ok(ret)
}

fn parse_input2(filename: &str) -> Result<(i64, i64)> {
    let input = std::fs::read_to_string(filename)?;
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let ret = (time, distance);
    Ok(ret)
}

#[test_case("inputs/example-06-1.txt" => matches Ok(71503))]
#[test_case("inputs/input-06.txt" => matches Ok(35349468))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let input = parse_input2(filename)?;
    let (time, distance) = input;
    // let mut ways = 0;
    // for dur in 0..time {
    //     let dist = (time - dur) * dur;
    //     if dist > distance {
    //         ways += 1;
    //     }
    // }
    let (x1, x2) = find_roots(-1.0, time as f64, -1.0 * distance as f64);
    let ways = prev_int(x2) - next_int(x1) + 1;
    Ok(ways)
}
