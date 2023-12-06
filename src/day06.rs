use anyhow::Result;
use test_case::test_case;

fn parse_input(filename: &str) -> Result<Vec<(i64, i64)>> {
    let input = std::fs::read_to_string(filename)?;
    let mut lines = input.lines();
    let times = lines.next().unwrap().split_whitespace().skip(1).map(|s| s.parse::<i64>().unwrap());
    let distances = lines.next().unwrap().split_whitespace().skip(1).map(|s| s.parse::<i64>().unwrap());
    let ret = times.zip(distances).collect::<Vec<_>>();
    return Ok(ret);
}

#[test_case("inputs/example-06-1.txt" => matches Ok(288))]
#[test_case("inputs/input-06.txt" => matches Ok(1710720))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let mut ret = 1;
    for (time, distance) in input {
        let mut ways = 0;
        for dur in 0..time {
            let dist = (time - dur) * dur;
            if dist > distance {
                ways += 1;
            }
        }
        ret *= ways;
    }
    Ok(ret)
}

fn parse_input2(filename: &str) -> Result<(i64, i64)> {
    let input = std::fs::read_to_string(filename)?;
    let mut lines = input.lines();
    let time = lines.next().unwrap().split_whitespace().skip(1).collect::<String>().parse::<i64>().unwrap();
    let distance = lines.next().unwrap().split_whitespace().skip(1).collect::<String>().parse::<i64>().unwrap();
    let ret = (time, distance);
    return Ok(ret);
}

#[test_case("inputs/example-06-1.txt" => matches Ok(71503))]
#[test_case("inputs/input-06.txt" => matches Ok(35349468))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let input = parse_input2(filename)?;
    let (time, distance) = input;
    let mut ways = 0;
    for dur in 0..time {
        let dist = (time - dur) * dur;
        if dist > distance {
            ways += 1;
        }
    }
    Ok(ways)
}
