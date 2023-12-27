use std::collections::HashMap;

use anyhow::Result;
use pathfinding::matrix::Matrix;
use test_case::test_case;

fn parse_input(filename: &str) -> Result<Matrix<char>> {
    let input = std::fs::read_to_string(filename)?;
    let ret = Matrix::from_rows(input.lines().filter(|l| !l.is_empty()).map(|l| l.chars()))?;
    Ok(ret)
}

#[test_case("inputs/example-03-1.txt" => matches Ok(4361))]
#[test_case("inputs/input-03.txt" => matches Ok(557705))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let mut total = 0;
    for row in 0..input.rows {
        let mut num = 0;
        let mut adj = false;
        for col in 0..input.columns {
            match input[(row, col)] {
                c @ ('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9') => {
                    num = num * 10 + (c as i64 - '0' as i64);
                    adj = adj
                        || input.neighbours((row, col), true).any(|p| {
                            !matches!(
                                input[p],
                                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.'
                            )
                        });
                }
                _ => {
                    if adj {
                        total += num;
                    }
                    num = 0;
                    adj = false;
                }
            }
        }
        if adj {
            total += num;
        }
    }
    Ok(total)
}

#[test_case("inputs/example-03-1.txt" => matches Ok(467835))]
#[test_case("inputs/input-03.txt" => matches Ok(84266818))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let mut potgears = HashMap::new();
    for row in 0..input.rows {
        let mut num = 0;
        let mut gearvec = Vec::new();
        for col in 0..input.columns {
            match input[(row, col)] {
                c @ ('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9') => {
                    num = num * 10 + (c as i64 - '0' as i64);
                    for gearpos in input
                        .neighbours((row, col), true)
                        .filter(|p| input[*p] == '*')
                    {
                        if !gearvec.contains(&gearpos) {
                            gearvec.push(gearpos);
                        }
                    }
                }
                _ => {
                    if num > 0 && !gearvec.is_empty() {
                        for gearpos in gearvec.drain(..) {
                            potgears.entry(gearpos).or_insert(Vec::new()).push(num);
                        }
                    }
                    num = 0;
                }
            }
        }
        if num > 0 && !gearvec.is_empty() {
            for gearpos in gearvec.drain(..) {
                potgears.entry(gearpos).or_insert(Vec::new()).push(num);
            }
        }
    }
    let mut total = 0;
    for nums in potgears.values() {
        if nums.len() == 2 {
            total += nums[0] * nums[1];
        }
    }
    Ok(total)
}
