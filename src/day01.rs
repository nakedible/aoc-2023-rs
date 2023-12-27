use anyhow::Result;
use test_case::test_case;

fn match_digit<const TEXT: bool>(input: &[u8]) -> Option<i64> {
    match input {
        [b'0', ..] => Some(0),
        [b'1', ..] => Some(1),
        [b'2', ..] => Some(2),
        [b'3', ..] => Some(3),
        [b'4', ..] => Some(4),
        [b'5', ..] => Some(5),
        [b'6', ..] => Some(6),
        [b'7', ..] => Some(7),
        [b'8', ..] => Some(8),
        [b'9', ..] => Some(9),
        [b'o', b'n', b'e', ..] if TEXT => Some(1),
        [b't', b'w', b'o', ..] if TEXT => Some(2),
        [b't', b'h', b'r', b'e', b'e', ..] if TEXT => Some(3),
        [b'f', b'o', b'u', b'r', ..] if TEXT => Some(4),
        [b'f', b'i', b'v', b'e', ..] if TEXT => Some(5),
        [b's', b'i', b'x', ..] if TEXT => Some(6),
        [b's', b'e', b'v', b'e', b'n', ..] if TEXT => Some(7),
        [b'e', b'i', b'g', b'h', b't', ..] if TEXT => Some(8),
        [b'n', b'i', b'n', b'e', ..] if TEXT => Some(9),
        _ => None,
    }
}

fn parse_row<const TEXT: bool>(input: &[u8]) -> (i64, i64) {
    let first = (0..input.len())
        .find_map(|i| match_digit::<TEXT>(&input[i..]))
        .unwrap();
    let last = (0..input.len())
        .rev()
        .find_map(|i| match_digit::<TEXT>(&input[i..]))
        .unwrap();
    (first, last)
}

fn parse_input<const TEXT: bool>(filename: &str) -> Result<Vec<(i64, i64)>> {
    let input = std::fs::read(filename)?;
    let rows = input
        .strip_suffix(b"\n")
        .unwrap()
        .split(|b| *b == b'\n')
        .map(parse_row::<TEXT>)
        .collect();
    Ok(rows)
}

#[test_case("inputs/example-01-1.txt" => matches Ok(142))]
#[test_case("inputs/input-01.txt" => matches Ok(54630))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let input = parse_input::<false>(filename)?;
    let ret = input
        .iter()
        .map(|(first, last)| first * 10 + last)
        .sum::<i64>();
    Ok(ret)
}

#[test_case("inputs/example-01-2.txt" => matches Ok(281))]
#[test_case("inputs/input-01.txt" => matches Ok(54770))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let input = parse_input::<true>(filename)?;
    let ret = input
        .iter()
        .map(|(first, last)| first * 10 + last)
        .sum::<i64>();
    Ok(ret)
}
