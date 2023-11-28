use anyhow::Result;
use chumsky::prelude::*;
use test_case::test_case;

use crate::util::*;

fn parser() -> impl Parser<char, Vec<Vec<i64>>, Error = Simple<char>> {
    let nums = integer().separated_by(just('\n')).allow_trailing().collect();
    let groups = nums.separated_by(just('\n')).collect();
    groups.then_ignore(end())
}

fn parse_input(filename: &str) -> Result<Vec<Vec<i64>>> {
    let input = std::fs::read_to_string(filename)?;
    parser().parse(input).map_err(errstr)
}

#[test_case("inputs/input-01.txt" => matches Ok(1))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let _ = parse_input(filename)?;
    anyhow::bail!("oh no")
}
