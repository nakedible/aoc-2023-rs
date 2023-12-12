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
    let ret = input.lines().map(|line| {
        let (springs, groups) = line.split_once(" ").unwrap();
        let springs = springs.chars().map(|c| match c {
            '.' => Spring::Ope,
            '#' => Spring::Dam,
            '?' => Spring::Unk,
            _ => unreachable!(),
        }).collect::<Vec<_>>();
        let groups = groups.split(",").map(str::parse::<i64>).collect::<Result<Vec<_>, _>>()?;
        Ok((springs, groups))
    }).collect();
    return ret;
}

fn count_springs(cache: &mut HashMap<(Vec<Spring>, Vec<i64>, i64), i64>, springs: &[Spring], groups: &[i64], curdam: i64) -> i64 {
    if let Some(ret) = cache.get(&(springs.to_vec(), groups.to_vec(), curdam)) {
        return *ret;
    }
    if springs.is_empty() {
        if groups.len() == 1 && groups[0] == curdam {
            return 1;
        } else if groups.is_empty() && curdam == 0 {
            return 1;
        } else {
            return 0;
        }
    }
    let mut ret = 0;
    if let Spring::Ope | Spring::Unk = springs[0] {
        if curdam > 0 {
            if !groups.is_empty() && groups[0] == curdam {
                ret += count_springs(cache, &springs[1..], &groups[1..], 0);
            }
        } else {
            ret += count_springs(cache, &springs[1..], groups, 0);
        }
    }
    if let Spring::Dam | Spring::Unk = springs[0] {
        ret += count_springs(cache, &springs[1..], groups, curdam + 1);
    }
    cache.insert((springs.to_vec(), groups.to_vec(), curdam), ret);
    ret
}

#[test_case("inputs/example-12-1.txt" => matches Ok(21))]
#[test_case("inputs/input-12.txt" => matches Ok(7916))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let mut ret = 0;
    let mut cache = HashMap::new();
    for (springs, groups) in input {
        let count = count_springs(&mut cache, &springs, &groups, 0);
        //println!("{}", count);
        ret += count;
    }
    Ok(ret)
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

#[test_case("inputs/example-12-1.txt" => matches Ok(525152))]
#[test_case("inputs/input-12.txt" => matches Ok(37366887898686))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let mut ret = 0;
    let mut cache = HashMap::new();
    for (springs, groups) in input {
        let (springs, groups) = unfold(&springs, &groups);
        let count = count_springs(&mut cache, &springs, &groups, 0);
        //println!("{}", count);
        ret += count;
    }
    Ok(ret)
}
