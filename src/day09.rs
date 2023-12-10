use anyhow::Result;
use test_case::test_case;

fn parse_input(filename: &str) -> Result<Vec<Vec<i64>>> {
    let input = std::fs::read_to_string(filename)?;
    let ret = input
        .lines()
        .map(|l| l.split(" ").map(|v| v.parse::<i64>().unwrap()).collect())
        .collect();
    return Ok(ret);
}

fn derive(row: &Vec<i64>) -> Vec<i64> {
    row.windows(2).map(|w| w[1] - w[0]).collect()
}

fn predict_next(row: &Vec<i64>) -> i64 {
    if row.iter().all(|&v| v == 0) {
        0
    } else {
        row.last().unwrap() + predict_next(&derive(row))
    }
}

#[test_case("inputs/example-09-1.txt" => matches Ok(114))]
#[test_case("inputs/input-09.txt" => matches Ok(2075724761))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let mut total = 0;
    for row in input {
        total += predict_next(&row);
    }
    Ok(total)
}

fn predict_prev(row: &Vec<i64>) -> i64 {
    if row.iter().all(|&v| v == 0) {
        0
    } else {
        row.first().unwrap() - predict_prev(&derive(row))
    }
}

#[test_case("inputs/example-09-1.txt" => matches Ok(2))]
#[test_case("inputs/input-09.txt" => matches Ok(1072))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let mut total = 0;
    for row in input {
        total += predict_prev(&row);
    }
    Ok(total)
}
