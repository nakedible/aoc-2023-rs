use anyhow::Result;
use std::collections::HashMap;
use test_case::test_case;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Ope,
    Dam,
    Unk,
}

fn parse_input(filename: &str) -> Result<Vec<(Vec<Spring>, Vec<i64>)>> {
    let input = std::fs::read_to_string(filename)?;
    let ret = input
        .lines()
        .map(|line| {
            let (springs, groups) = line.split_once(" ").unwrap();
            let springs = springs
                .chars()
                .map(|c| match c {
                    '.' => Spring::Ope,
                    '#' => Spring::Dam,
                    '?' => Spring::Unk,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();
            let groups = groups
                .split(",")
                .map(str::parse::<i64>)
                .collect::<Result<Vec<_>, _>>()?;
            Ok((springs, groups))
        })
        .collect();
    return ret;
}

fn count_springs<'a>(
    cache: &mut HashMap<(&'a [Spring], &'a [i64], i64), i64>,
    springs: &'a [Spring],
    groups: &'a [i64],
    curdam: i64,
    minlen: i64,
) -> i64 {
    match (springs.len() as i64, groups.len() as i64, groups.first()) {
        (0, 1, Some(c)) if *c == curdam => return 1,
        (0, 0, _) if curdam == 0 => return 1,
        (0, _, _) => return 0,
        (_, _, Some(c)) if curdam > *c => return 0,
        (_, _, None) if curdam > 0 => return 0,
        (s, _, _) if s + curdam < minlen => return 0,
        _ => (),
    }
    if let Some(ret) = cache.get(&(springs, groups, curdam)) {
        return *ret;
    }
    let mut ret = 0;
    if let Spring::Ope | Spring::Unk = springs[0] {
        if curdam > 0 {
            if !groups.is_empty() && groups[0] == curdam {
                ret += count_springs(cache, &springs[1..], &groups[1..], 0, minlen - curdam - 1);
            }
        } else {
            ret += count_springs(cache, &springs[1..], groups, 0, minlen);
        }
    }
    if let Spring::Dam | Spring::Unk = springs[0] {
        ret += count_springs(cache, &springs[1..], groups, curdam + 1, minlen);
    }
    cache.insert((springs, groups, curdam), ret);
    ret
}

fn unfold(springs: &Vec<Spring>, groups: &Vec<i64>) -> (Vec<Spring>, Vec<i64>) {
    let springs = springs
        .iter()
        .chain(Some(&Spring::Unk))
        .chain(springs.iter())
        .chain(Some(&Spring::Unk))
        .chain(springs.iter())
        .chain(Some(&Spring::Unk))
        .chain(springs.iter())
        .chain(Some(&Spring::Unk))
        .chain(springs.iter())
        .cloned()
        .collect::<Vec<_>>();
    let groups = groups
        .iter()
        .chain(groups.iter())
        .chain(groups.iter())
        .chain(groups.iter())
        .chain(groups.iter())
        .cloned()
        .collect::<Vec<_>>();
    (springs, groups)
}

#[test_case("inputs/example-12-1.txt", false => matches Ok(21))]
#[test_case("inputs/input-12.txt", false => matches Ok(7916))]
#[test_case("inputs/example-12-1.txt", true => matches Ok(525152))]
#[test_case("inputs/input-12.txt", true => matches Ok(37366887898686))]
pub fn puzzle1and2(filename: &str, unfolded: bool) -> Result<i64> {
    let mut input = parse_input(filename)?;
    if unfolded {
        input = input
            .iter()
            .map(|(springs, groups)| unfold(&springs, &groups))
            .collect();
    }
    let mut ret = 0;
    for (springs, groups) in input.iter() {
        let minlen = groups.iter().sum::<i64>() + groups.len() as i64 - 1;
        let count = count_springs(&mut HashMap::new(), springs, groups, 0, minlen);
        ret += count;
    }
    Ok(ret)
}
