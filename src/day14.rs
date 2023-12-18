use anyhow::Result;
use pathfinding::matrix::Matrix;
use test_case::test_case;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    R,
    C,
    E,
}

fn parse_input(filename: &str) -> Result<Matrix<Rock>> {
    let input = std::fs::read_to_string(filename)?;
    let ret = Matrix::from_rows(input.lines().filter(|l| !l.is_empty()).map(|l| {
        l.chars().map(|c| match c {
            'O' => Rock::R,
            '#' => Rock::C,
            '.' => Rock::E,
            _ => unreachable!(),
        })
    }))?;
    return Ok(ret);
}

#[allow(dead_code)]
fn print_matrix(matrix: &Matrix<Rock>) {
    for y in 0..matrix.rows {
        for x in 0..matrix.columns {
            print!(
                "{}",
                match matrix[(y, x)] {
                    Rock::R => 'O',
                    Rock::C => '#',
                    Rock::E => '.',
                }
            );
        }
        println!();
    }
}

fn roll(matrix: &mut Matrix<Rock>) {
    for x in 0..matrix.columns {
        let mut free_row = None;
        for y in 0..matrix.rows {
            match matrix[(y, x)] {
                Rock::R => {
                    if let Some(r) = free_row {
                        matrix[(y, x)] = Rock::E;
                        matrix[(r, x)] = Rock::R;
                        free_row = Some(r + 1);
                    }
                }
                Rock::C => {
                    free_row = None;
                }
                Rock::E => {
                    if free_row.is_none() {
                        free_row = Some(y);
                    }
                }
            }
        }
    }
}

#[test_case("inputs/example-14-1.txt" => matches Ok(136))]
#[test_case("inputs/input-14.txt" => matches Ok(108759))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let mut input = parse_input(filename)?;
    roll(&mut input);
    let ret = input
        .items()
        .filter(|(_, r)| **r == Rock::R)
        .map(|((y, _), _)| input.rows - y)
        .sum::<usize>() as i64;
    Ok(ret)
}

fn cycle(matrix: &mut Matrix<Rock>) {
    roll(matrix);
    matrix.rotate_cw(1);
    roll(matrix);
    matrix.rotate_cw(1);
    roll(matrix);
    matrix.rotate_cw(1);
    roll(matrix);
    matrix.rotate_cw(1);
}

fn find_loop(matrix: &mut Matrix<Rock>) -> (usize, usize) {
    let mut seen = std::collections::HashMap::new();
    for num in 1.. {
        cycle(matrix);
        if seen.contains_key(matrix) {
            return (seen[matrix], num);
        } else {
            seen.insert(matrix.clone(), num);
        }
    }
    unreachable!();
}

#[test_case("inputs/example-14-1.txt" => matches Ok(64))]
#[test_case("inputs/input-14.txt" => matches Ok(89089))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let mut input = parse_input(filename)?;
    let (start, end) = find_loop(&mut input);
    let rem = (1000000000 - end) % (end - start);
    //println!("start: {}, end: {}, rem: {}", start, end, rem);
    for _ in 0..rem {
        cycle(&mut input);
    }
    let ret = input
        .items()
        .filter(|(_, r)| **r == Rock::R)
        .map(|((y, _), _)| input.rows - y)
        .sum::<usize>() as i64;
    Ok(ret)
}
