use anyhow::Result;
use pathfinding::matrix::{directions, Matrix};
use std::collections::HashSet;
use test_case::test_case;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    G,
    S,
    O,
}

fn char_to_pipe(c: char) -> Pipe {
    match c {
        '|' => Pipe::NS,
        '-' => Pipe::EW,
        'L' => Pipe::NE,
        'J' => Pipe::NW,
        '7' => Pipe::SW,
        'F' => Pipe::SE,
        '.' => Pipe::G,
        'S' => Pipe::S,
        _ => panic!("Unknown pipe char: {}", c),
    }
}

fn parse_input(filename: &str) -> Result<Matrix<Pipe>> {
    let input = std::fs::read_to_string(filename)?;
    let ret = Matrix::from_rows(input.lines().map(|l| l.chars().map(char_to_pipe))).unwrap();
    return Ok(ret);
}

#[allow(dead_code)]
fn print_input(input: &Matrix<Pipe>) {
    for y in 0..input.rows {
        for x in 0..input.columns {
            print!(
                "{}",
                match input[(y, x)] {
                    Pipe::NS => '┃',
                    Pipe::EW => '━',
                    Pipe::NE => '┗',
                    Pipe::NW => '┛',
                    Pipe::SW => '┓',
                    Pipe::SE => '┏',
                    Pipe::G => '▦',
                    Pipe::S => '╳',
                    Pipe::O => ' ',
                }
            );
        }
        println!();
    }
}

fn next_pipe(dir: (isize, isize), cur: Pipe) -> Option<(isize, isize)> {
    match (dir, cur) {
        (directions::N, Pipe::NS) => Some(directions::N),
        (directions::S, Pipe::NS) => Some(directions::S),
        (directions::E, Pipe::EW) => Some(directions::E),
        (directions::W, Pipe::EW) => Some(directions::W),
        (directions::S, Pipe::NE) => Some(directions::E),
        (directions::W, Pipe::NE) => Some(directions::N),
        (directions::E, Pipe::NW) => Some(directions::N),
        (directions::S, Pipe::NW) => Some(directions::W),
        (directions::N, Pipe::SW) => Some(directions::W),
        (directions::E, Pipe::SW) => Some(directions::S),
        (directions::N, Pipe::SE) => Some(directions::E),
        (directions::W, Pipe::SE) => Some(directions::S),
        _ => None,
    }
}

#[test_case("inputs/example-10-1.txt" => matches Ok(8))]
#[test_case("inputs/input-10.txt" => matches Ok(6927))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let (startpos, _) = input.items().find(|&(_, &v)| v == Pipe::S).unwrap();
    let mut curpos = startpos;
    let mut curdir = directions::N;
    for dir in directions::DIRECTIONS_4 {
        if input
            .move_in_direction(startpos, dir)
            .and_then(|pos| next_pipe(dir, input[pos]))
            .is_some()
        {
            curdir = dir;
            curpos = input.move_in_direction(startpos, dir).unwrap();
            break;
        }
    }
    let mut count = 0;
    while curpos != startpos {
        count += 1;
        curdir = next_pipe(curdir, input[curpos]).unwrap();
        curpos = input.move_in_direction(curpos, curdir).unwrap();
    }
    let ret = (count + 1) / 2;
    Ok(ret)
}

fn expand_pipes(input: &Matrix<Pipe>) -> Matrix<Pipe> {
    let mut ret = Matrix::new(input.rows * 3, input.columns * 3, Pipe::O);
    for row in 0..input.rows {
        for col in 0..input.columns {
            match input[(row, col)] {
                Pipe::NS => {
                    ret[(row * 3 + 0, col * 3 + 1)] = Pipe::NS;
                    ret[(row * 3 + 1, col * 3 + 1)] = Pipe::NS;
                    ret[(row * 3 + 2, col * 3 + 1)] = Pipe::NS;
                }
                Pipe::EW => {
                    ret[(row * 3 + 1, col * 3 + 0)] = Pipe::EW;
                    ret[(row * 3 + 1, col * 3 + 1)] = Pipe::EW;
                    ret[(row * 3 + 1, col * 3 + 2)] = Pipe::EW;
                }
                Pipe::NE => {
                    ret[(row * 3 + 0, col * 3 + 1)] = Pipe::NS;
                    ret[(row * 3 + 1, col * 3 + 1)] = Pipe::NE;
                    ret[(row * 3 + 1, col * 3 + 2)] = Pipe::EW;
                }
                Pipe::NW => {
                    ret[(row * 3 + 0, col * 3 + 1)] = Pipe::NS;
                    ret[(row * 3 + 1, col * 3 + 1)] = Pipe::NW;
                    ret[(row * 3 + 1, col * 3 + 0)] = Pipe::EW;
                }
                Pipe::SW => {
                    ret[(row * 3 + 1, col * 3 + 0)] = Pipe::EW;
                    ret[(row * 3 + 1, col * 3 + 1)] = Pipe::SW;
                    ret[(row * 3 + 2, col * 3 + 1)] = Pipe::NS;
                }
                Pipe::SE => {
                    ret[(row * 3 + 1, col * 3 + 1)] = Pipe::SE;
                    ret[(row * 3 + 1, col * 3 + 2)] = Pipe::EW;
                    ret[(row * 3 + 2, col * 3 + 1)] = Pipe::NS;
                }
                Pipe::G => {
                    ret[(row * 3 + 1, col * 3 + 1)] = Pipe::G;
                }
                Pipe::S => {
                    ret[(row * 3 + 0, col * 3 + 0)] = Pipe::S;
                    ret[(row * 3 + 0, col * 3 + 1)] = Pipe::S;
                    ret[(row * 3 + 0, col * 3 + 2)] = Pipe::S;
                    ret[(row * 3 + 1, col * 3 + 0)] = Pipe::S;
                    ret[(row * 3 + 1, col * 3 + 1)] = Pipe::S;
                    ret[(row * 3 + 1, col * 3 + 2)] = Pipe::S;
                    ret[(row * 3 + 2, col * 3 + 0)] = Pipe::S;
                    ret[(row * 3 + 2, col * 3 + 1)] = Pipe::S;
                    ret[(row * 3 + 2, col * 3 + 2)] = Pipe::S;
                }
                Pipe::O => {}
            }
        }
    }
    ret
}

#[test_case("inputs/example-10-1.txt" => matches Ok(1))]
#[test_case("inputs/input-10.txt" => matches Ok(467))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let mut input = parse_input(filename)?;
    // find loop
    let (startpos, _) = input.items().find(|&(_, &v)| v == Pipe::S).unwrap();
    let mut looppipes = HashSet::new();
    looppipes.insert(startpos);
    let mut curpos = startpos;
    let mut curdir = directions::N;
    for dir in directions::DIRECTIONS_4 {
        if input
            .move_in_direction(startpos, dir)
            .and_then(|pos| next_pipe(dir, input[pos]))
            .is_some()
        {
            curdir = dir;
            curpos = input.move_in_direction(startpos, dir).unwrap();
            break;
        }
    }
    while curpos != startpos {
        looppipes.insert(curpos);
        curdir = next_pipe(curdir, input[curpos]).unwrap();
        curpos = input.move_in_direction(curpos, curdir).unwrap();
    }
    // delete everything else except loop
    for pos in input.keys() {
        if !looppipes.contains(&pos) {
            input[pos] = Pipe::G;
        }
    }
    // expand
    let mut expanded = expand_pipes(&input);
    // nuke reachable from edge
    let outer = expanded.bfs_reachable((0,0), false, |pos| expanded[pos] == Pipe::G || expanded[pos] == Pipe::O);
    for o in outer {
        expanded[o] = Pipe::O;
    }
    // count grounds
    let ret = expanded.values().filter(|&&v| v == Pipe::G).count() as i64;
    Ok(ret)
}
