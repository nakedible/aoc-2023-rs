use anyhow::Result;
use test_case::test_case;

fn parse_input(filename: &str) -> Result<Vec<(i64, i64)>> {
    let input = std::fs::read_to_string(filename)?;
    let ret = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| (x as i64, y as i64))
        })
        .collect();
    Ok(ret)
}

fn expand(input: &mut [(i64, i64)], count: i64) {
    let mut prevx = 0;
    let mut xinc = 0;
    input.sort_by_key(|(x, _)| *x);
    for (x, _) in input.iter_mut() {
        if *x != prevx {
            xinc += (*x - prevx - 1) * count;
            prevx = *x;
        }
        *x += xinc;
    }
    let mut prevy = 0;
    let mut yinc = 0;
    input.sort_by_key(|(_, y)| *y);
    for (_, y) in input.iter_mut() {
        if *y != prevy {
            yinc += (*y - prevy - 1) * count;
            prevy = *y;
        }
        *y += yinc;
    }
}

#[test_case("inputs/example-11-1.txt", 1 => matches Ok(374))]
#[test_case("inputs/example-11-1.txt", 9 => matches Ok(1030))]
#[test_case("inputs/example-11-1.txt", 99 => matches Ok(8410))]
#[test_case("inputs/input-11.txt", 999999 => matches Ok(827009909817))]
pub fn puzzle1and2(filename: &str, count: i64) -> Result<i64> {
    let mut input = parse_input(filename)?;
    expand(&mut input, count);
    let tot = input
        .iter()
        .enumerate()
        .flat_map(|(i, &(g1x, g1y))| {
            input[i + 1..]
                .iter()
                .map(move |&(g2x, g2y)| (g1x - g2x).abs() + (g1y - g2y).abs())
        })
        .sum();
    Ok(tot)
}
