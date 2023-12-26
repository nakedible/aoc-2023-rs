use anyhow::Result;
use std::collections::HashSet;
use test_case::test_case;

type Point = (i64, i64, i64);
type Block = (Point, Point);

fn parse_input(filename: &str) -> Result<Vec<Block>> {
    let input = std::fs::read_to_string(filename)?;
    let ret = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (beg, end) = l.split_once('~').unwrap();
            let (x1, beg) = beg.split_once(',').unwrap();
            let (y1, z1) = beg.split_once(',').unwrap();
            let (x2, end) = end.split_once(',').unwrap();
            let (y2, z2) = end.split_once(',').unwrap();
            let (x1, y1, z1) = (x1.parse().unwrap(), y1.parse().unwrap(), z1.parse().unwrap());
            let (x2, y2, z2) = (x2.parse().unwrap(), y2.parse().unwrap(), z2.parse().unwrap());
            assert!(x1 <= x2 && y1 <= y2 && z1 <= z2);
            ((x1, y1, z1), (x2, y2, z2))
        })
        .collect();
    return Ok(ret);
}

fn one_down(block: Block) -> Option<Block> {
    let ((x1, y1, z1), (x2, y2, z2)) = block;
    if z1 == 0 {
        None
    } else {
        Some(((x1, y1, z1 - 1), (x2, y2, z2 - 1)))
    }
}

fn intersects(a: Block, b: Block) -> bool {
    let ((x1, y1, z1), (x2, y2, z2)) = a;
    let ((x3, y3, z3), (x4, y4, z4)) = b;
    x1 <= x4 && x2 >= x3 && y1 <= y4 && y2 >= y3 && z1 <= z4 && z2 >= z3
}

fn fall_blocks(input: &mut Vec<(Point, Point)>) {
    for i in 0..input.len() {
        let mut cur = input[i];
        while let Some(fall_block) = one_down(cur) {
            if (0..i).rev().any(|j| intersects(fall_block, input[j])) {
                break;
            }
            cur = fall_block;
        }
        input[i] = cur;
    }
}

fn would_fall(input: &Vec<(Point, Point)>, skip: usize) -> bool {
    for i in 0..input.len() {
        let cur = input[i];
        if let Some(fall_block) = one_down(cur) {
            if (0..i).rev().all(|j| j == skip || !intersects(fall_block, input[j])) {
                return true;
            }
        }
    }
    false
}

#[test_case("inputs/example-22-1.txt" => matches Ok(5))]
#[test_case("inputs/input-22.txt" => matches Ok(424))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let mut input = parse_input(filename)?;
    input.sort_by_key(|&((_, _, z1), _)| z1);
    fall_blocks(&mut input);
    input.sort_by_key(|&((_, _, z1), _)| z1);
    let ret = (0..input.len()).filter(|&i| !would_fall(&input, i)).count() as i64;
    Ok(ret)
}

fn map_supports(input: &Vec<(Point, Point)>) -> Vec<Vec<usize>> {
    let mut ret = Vec::new();
    for i in 0..input.len() {
        let cur = input[i];
        if let Some(fall_block) = one_down(cur) {
            ret.push((0..i).rev().filter(|&j| intersects(fall_block, input[j])).collect());
        } else {
            ret.push(Vec::new());
        }
    }
    ret
}

fn count_fall(supports: &Vec<Vec<usize>>, skip: usize) -> usize {
    let mut dis = HashSet::new();
    dis.insert(skip);
    for i in 0..supports.len() {
        if !supports[i].is_empty() && supports[i].iter().all(|j| dis.contains(&j)) {
            dis.insert(i);
        }
    }
    dis.len() - 1
}

#[test_case("inputs/example-22-1.txt" => matches Ok(7))]
#[test_case("inputs/input-22.txt" => matches Ok(55483))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let mut input = parse_input(filename)?;
    input.sort_by_key(|&((_, _, z1), _)| z1);
    fall_blocks(&mut input);
    input.sort_by_key(|&((_, _, z1), _)| z1);
    let supports = map_supports(&input);
    let ret = (0..input.len()).map(|i| count_fall(&supports, i)).sum::<usize>() as i64;
    Ok(ret)
}
