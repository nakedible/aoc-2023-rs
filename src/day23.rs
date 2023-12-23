
use anyhow::Result;
use std::collections::{VecDeque, HashSet};
use pathfinding::matrix::{directions, Matrix};
use petgraph::graphmap::DiGraphMap;
use test_case::test_case;

fn parse_input(filename: &str) -> Result<Matrix<char>> {
    let input = std::fs::read_to_string(filename)?;
    let ret = Matrix::from_rows(input.lines().filter(|l| !l.is_empty()).map(|l| l.chars()))?;
    return Ok(ret);
}

fn neighbours(map: &Matrix<char>, pos: (usize, usize)) -> impl IntoIterator<Item = ((usize, usize), i64)> + '_ {
    let dirs = match map[pos] {
        '>' => &[directions::E][..],
        '^' => &[directions::N][..],
        '<' => &[directions::W][..],
        'v' => &[directions::S][..],
        '.' => &directions::DIRECTIONS_4[..],
        _ => unreachable!(),
    };
    dirs
        .iter()
        .flat_map(move |dir| map.move_in_direction(pos, *dir))
        .filter(|&p| match map[p] {
            '.' => true,
            '^' | '>' | 'v' | '<' => true,
            _ => false,
        })
        .map(move |p| (p, -1))
}

#[test_case("inputs/example-23-1.txt" => matches Ok(94))]
#[test_case("inputs/input-23.txt" => matches Ok(1930))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let start = (0, 1);
    let goal = (input.rows - 1, input.columns - 2);
    let mut branches = VecDeque::new();
    branches.push_back((start, start, 0));
    let mut max_cost = 0i64;
    while let Some((prev, next, cost)) = branches.pop_front() {
        if next == goal {
            max_cost = max_cost.max(cost);
            continue;
        }
        neighbours(&input, next).into_iter().filter(|&(p, _)| p != prev).for_each(|(p, _)| branches.push_back((next, p, cost + 1)));
    }
    let ret = max_cost;
    Ok(ret)
}

fn neighbours2(map: &Matrix<char>, pos: (usize, usize)) -> impl IntoIterator<Item = ((usize, usize), i64)> + '_ {
    let dirs = &directions::DIRECTIONS_4[..];
    dirs
        .iter()
        .flat_map(move |dir| map.move_in_direction(pos, *dir))
        .filter(|&p| match map[p] {
            '.' => true,
            '^' | '>' | 'v' | '<' => true,
            _ => false,
        })
        .map(move |p| (p, -1))
}

fn simplify(input: &Matrix<char>) -> DiGraphMap<(usize, usize), i64> {
    let start = (0, 1);
    let goal = (input.rows - 1, input.columns - 2);
    let mut branches = VecDeque::new();
    branches.push_back((start, start, start, 0));
    let mut graph = DiGraphMap::new();
    while let Some((prev_branch, prev, next, cost)) = branches.pop_front() {
        if next == goal {
            graph.add_edge(prev_branch, next, cost);
            continue;
        }
        let posses: Vec<(usize, usize)> = neighbours2(&input, next).into_iter().filter(|&(p, _)| p != prev).map(|(p, _)| p).collect();
        if posses.len() == 1 {
            branches.push_back((prev_branch, next, posses[0], cost + 1));
        } else {
            if let Some(_) = graph.add_edge(prev_branch, next, cost) {
                continue;
            }
            for p in posses {
                branches.push_back((next, next, p, 1));
            }
        }
    }
    graph
}

#[test_case("inputs/example-23-1.txt" => matches Ok(154))]
#[test_case("inputs/input-23.txt" => matches Ok(6230))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let graph = simplify(&input);
    let mut branches = VecDeque::new();
    let start = (0, 1);
    let goal = (input.rows - 1, input.columns - 2);
    let mut seen = HashSet::new();
    seen.insert(start);
    branches.push_back((start, 0, seen));
    let mut max_cost = 0i64;
    while let Some((node, cost, seen)) = branches.pop_front() {
        if node == goal {
            max_cost = max_cost.max(cost);
            continue;
        }
        for (_, next, next_cost) in graph.edges(node) {
            if seen.contains(&next) {
                continue;
            }
            let mut seen = seen.clone();
            seen.insert(next);
            branches.push_back((next, cost + next_cost, seen));
        }
    }
    let ret = max_cost;
    Ok(ret)
}
