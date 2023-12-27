use anyhow::Result;
use pathfinding::matrix::{directions, Matrix};
use std::collections::HashSet;
use test_case::test_case;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Slash,
    Backslash,
    Versplit,
    Horsplit,
}

fn parse_input(filename: &str) -> Result<Matrix<Tile>> {
    let input = std::fs::read_to_string(filename)?;
    let ret = Matrix::from_rows(input.lines().filter(|l| !l.is_empty()).map(|l| {
        l.chars().map(|c| match c {
            '.' => Tile::Empty,
            '/' => Tile::Slash,
            '\\' => Tile::Backslash,
            '|' => Tile::Versplit,
            '-' => Tile::Horsplit,
            _ => unreachable!(),
        })
    }))?;
    Ok(ret)
}

fn shoot_ray(map: &Matrix<Tile>, pos: (usize, usize), dir: (isize, isize)) -> i64 {
    let mut seent = HashSet::new();
    let mut rays = vec![(Some(pos), dir)];
    while let Some((mut pos, mut dir)) = rays.pop() {
        while let Some(p) = pos {
            if !seent.insert((p, dir)) {
                break;
            }
            match map[p] {
                Tile::Empty => (),
                Tile::Slash if dir == directions::E => dir = directions::N,
                Tile::Slash if dir == directions::N => dir = directions::E,
                Tile::Slash if dir == directions::W => dir = directions::S,
                Tile::Slash if dir == directions::S => dir = directions::W,
                Tile::Slash => unreachable!(),
                Tile::Backslash if dir == directions::E => dir = directions::S,
                Tile::Backslash if dir == directions::N => dir = directions::W,
                Tile::Backslash if dir == directions::W => dir = directions::N,
                Tile::Backslash if dir == directions::S => dir = directions::E,
                Tile::Backslash => unreachable!(),
                Tile::Versplit if dir == directions::E || dir == directions::W => {
                    rays.push((map.move_in_direction(p, directions::N), directions::N));
                    rays.push((map.move_in_direction(p, directions::S), directions::S));
                    break;
                }
                Tile::Versplit => (),
                Tile::Horsplit if dir == directions::N || dir == directions::S => {
                    rays.push((map.move_in_direction(p, directions::E), directions::E));
                    rays.push((map.move_in_direction(p, directions::W), directions::W));
                    break;
                }
                Tile::Horsplit => (),
            }
            pos = map.move_in_direction(p, dir);
        }
    }
    let posmap = seent.iter().map(|(p, _)| p).collect::<HashSet<_>>();
    posmap.len() as i64
}

#[test_case("inputs/example-16-1.txt" => matches Ok(46))]
#[test_case("inputs/input-16.txt" => matches Ok(6978))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let ret = shoot_ray(&input, (0, 0), directions::E);
    Ok(ret)
}

#[test_case("inputs/example-16-1.txt" => matches Ok(51))]
#[test_case("inputs/input-16.txt" => matches Ok(7315))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let mut max = 0;
    for y in 0..input.rows {
        max = std::cmp::max(max, shoot_ray(&input, (y, 0), directions::E));
        max = std::cmp::max(
            max,
            shoot_ray(&input, (y, input.columns - 1), directions::W),
        );
    }
    for x in 0..input.columns {
        max = std::cmp::max(max, shoot_ray(&input, (0, x), directions::S));
        max = std::cmp::max(max, shoot_ray(&input, (input.rows - 1, x), directions::N));
    }
    Ok(max)
}
