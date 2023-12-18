use anyhow::Result;
use pathfinding::directed::astar::astar;
use pathfinding::matrix::{directions, Matrix};
use test_case::test_case;

fn parse_input(filename: &str) -> Result<Matrix<u8>> {
    let input = std::fs::read_to_string(filename)?;
    let ret = Matrix::from_rows(
        input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().map(|c| c as u8 - '0' as u8)),
    )?;
    return Ok(ret);
}

type State = ((usize, usize), (isize, isize), usize);

fn successors1(input: &Matrix<u8>, state: State) -> impl IntoIterator<Item = (State, usize)> + '_ {
    let (pos, dir, count) = state;
    [
        (directions::N, directions::S),
        (directions::S, directions::N),
        (directions::E, directions::W),
        (directions::W, directions::E),
    ]
    .iter()
    .map(move |&(d, o)| {
        if o == dir {
            None
        } else if d == dir && count >= 3 {
            None
        } else if let Some(p) = input.move_in_direction(pos, d) {
            let c = if d == dir { count + 1 } else { 1 };
            Some(((p, d, c), input[p] as usize))
        } else {
            None
        }
    })
    .flatten()
}

fn heuristic(pos: (usize, usize), goal: (usize, usize)) -> usize {
    pos.0.abs_diff(goal.0) + pos.1.abs_diff(goal.1)
}

fn success(pos: (usize, usize), goal: (usize, usize)) -> bool {
    pos == goal
}

#[test_case("inputs/example-17-1.txt" => matches Ok(102))]
#[test_case("inputs/input-17.txt" => matches Ok(1076))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let goal = (input.rows - 1, input.columns - 1);
    let (_, cost) = astar(
        &((0, 0), directions::W, 0),
        |&state| successors1(&input, state),
        |&state| heuristic(state.0, goal),
        |&state| success(state.0, goal),
    )
    .expect("no path found");
    Ok(cost as i64)
}

fn successors2(input: &Matrix<u8>, state: State) -> impl IntoIterator<Item = (State, usize)> + '_ {
    let (pos, dir, count) = state;
    [
        (directions::N, directions::S),
        (directions::S, directions::N),
        (directions::E, directions::W),
        (directions::W, directions::E),
    ]
    .iter()
    .map(move |&(d, o)| {
        if o == dir && count > 0 {
            None
        } else if d == dir && count >= 10 {
            None
        } else if d != dir && count > 0 && count < 4 {
            None
        } else if let Some(p) = input.move_in_direction(pos, d) {
            let c = if d == dir { count + 1 } else { 1 };
            Some(((p, d, c), input[p] as usize))
        } else {
            None
        }
    })
    .flatten()
}

#[test_case("inputs/example-17-1.txt" => matches Ok(94))]
#[test_case("inputs/input-17.txt" => matches Ok(1219))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let goal = (input.rows - 1, input.columns - 1);
    let (_path, cost) = astar(
        &((0, 0), directions::W, 0),
        |&state| successors2(&input, state),
        |&state| heuristic(state.0, goal),
        |&state| success(state.0, goal),
    )
    .unwrap();
    Ok(cost as i64)
}
