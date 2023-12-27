use anyhow::Result;
use pathfinding::matrix::Matrix;
use test_case::test_case;

fn parse_input(filename: &str) -> Result<Vec<Matrix<bool>>> {
    let input = std::fs::read_to_string(filename)?;
    let ret = input
        .split("\n\n")
        .map(|m| {
            Matrix::from_rows(m.lines().map(|l| {
                l.chars().map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => unreachable!(),
                })
            }))
            .unwrap()
        })
        .collect();
    Ok(ret)
}

fn find_mirror(input: &Matrix<bool>) -> Option<usize> {
    for y in 1..input.rows {
        for diff in 1..input.rows {
            if diff > y || y + diff > input.rows {
                if diff > 0 {
                    return Some(y);
                }
                break;
            }
            if (0..input.columns).any(|x| input[(y + diff - 1, x)] != input[(y - diff, x)]) {
                break;
            }
        }
    }
    None
}

#[test_case("inputs/example-13-1.txt" => matches Ok(405))]
#[test_case("inputs/input-13.txt" => matches Ok(33356))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let mut total = 0;
    for m in input {
        let count =
            (find_mirror(&m).unwrap_or(0) * 100) + find_mirror(&m.rotated_cw(1)).unwrap_or(0);
        total += count as i64;
    }
    Ok(total)
}

fn find_mirror_smudged(input: &Matrix<bool>) -> Option<usize> {
    for y in 1..input.rows {
        let mut smudged = false;
        for diff in 1..input.rows {
            if diff > y || y + diff > input.rows {
                if diff > 0 && smudged {
                    return Some(y);
                }
                break;
            }
            match (0..input.columns)
                .filter(|x| input[(y + diff - 1, *x)] != input[(y - diff, *x)])
                .count()
            {
                0 => (),
                1 if smudged => {
                    break;
                }
                1 => {
                    smudged = true;
                }
                _ => {
                    break;
                }
            }
        }
    }
    None
}

#[test_case("inputs/example-13-1.txt" => matches Ok(400))]
#[test_case("inputs/input-13.txt" => matches Ok(28475))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let mut total = 0;
    for m in input {
        let count = (find_mirror_smudged(&m).unwrap_or(0) * 100)
            + find_mirror_smudged(&m.rotated_cw(1)).unwrap_or(0);
        total += count as i64;
    }
    Ok(total)
}
