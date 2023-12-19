use anyhow::Result;
use std::collections::HashMap;
use test_case::test_case;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cat {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

type Part = [i64; 4];

type Id = u32;

#[derive(Debug)]
enum Rule {
    Lt(Cat, i64, Id),
    Gt(Cat, i64, Id),
    Al(Id),
}

fn parse_id(s: &str) -> Id {
    s.as_bytes().iter().fold(0, |acc, c| (acc << 8 | *c as u32))
}

#[allow(dead_code)]
fn print_id(id: Id) -> String {
    let mut ret = String::new();
    let mut id = id;
    while id != 0 {
        ret.push((id & 0xff) as u8 as char);
        id >>= 8;
    }
    ret.chars().rev().collect()
}

fn parse_cat(s: &str) -> Cat {
    match s {
        "x" => Cat::X,
        "m" => Cat::M,
        "a" => Cat::A,
        "s" => Cat::S,
        _ => unreachable!(),
    }
}

fn parse_rule(s: &str) -> Rule {
    if let Some((cat, rest)) = s.split_once('<') {
        let (val, dst) = rest.split_once(':').unwrap();
        let cat = parse_cat(cat);
        let val = val.parse().unwrap();
        let dst = parse_id(dst);
        Rule::Lt(cat, val, dst)
    } else if let Some((cat, rest)) = s.split_once('>') {
        let (val, dst) = rest.split_once(':').unwrap();
        let cat = parse_cat(cat);
        let val = val.parse().unwrap();
        let dst = parse_id(dst);
        Rule::Gt(cat, val, dst)
    } else {
        let dst = parse_id(s);
        Rule::Al(dst)
    }
}

fn parse_input(filename: &str) -> Result<(HashMap<Id, Vec<Rule>>, Vec<Part>)> {
    let input = std::fs::read_to_string(filename)?;
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    // Example: px{a<2006:qkq,m>2090:A,rfg}
    let workflows: HashMap<Id, Vec<Rule>> = workflows
        .lines()
        .map(|l| {
            let (id, rest) = l.split_once('{').unwrap();
            let id = parse_id(id);
            let rules = rest[..rest.len() - 1].split(',').map(parse_rule).collect();
            (id, rules)
        })
        .collect();
    // Example: {x=787,m=2655,a=1222,s=2876}
    let parts: Vec<Part> = parts
        .lines()
        .map(|l| {
            let mut ret = [0; 4];
            for p in l[1..l.len() - 1].split(',') {
                let (k, v) = p.split_once('=').unwrap();
                let v = v.parse().unwrap();
                ret[parse_cat(k) as usize] = v;
            }
            ret
        })
        .collect();
    return Ok((workflows, parts));
}

#[test_case("inputs/example-19-1.txt" => matches Ok(19114))]
#[test_case("inputs/input-19.txt" => matches Ok(456651))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let (workflows, parts) = parse_input(filename)?;
    let ret = parts
        .iter()
        .map(|part| {
            let mut id = parse_id("in");
            loop {
                let rules = workflows.get(&id).unwrap();
                for rule in rules {
                    match rule {
                        Rule::Lt(cat, val, dst) => {
                            if part[*cat as usize] < *val {
                                id = *dst;
                                break;
                            }
                        }
                        Rule::Gt(cat, val, dst) => {
                            if part[*cat as usize] > *val {
                                id = *dst;
                                break;
                            }
                        }
                        Rule::Al(dst) => {
                            id = *dst;
                            break;
                        }
                    }
                }
                if id == parse_id("A") {
                    return part.iter().sum();
                } else if id == parse_id("R") {
                    return 0;
                }
            }
        })
        .sum::<i64>();
    Ok(ret)
}

fn traverse_rules(workflows: &HashMap<Id, Vec<Rule>>, id: Id, mut inputs: [(i64, i64); 4]) -> i64 {
    if id == parse_id("A") {
        return inputs.iter().fold(1, |acc, (a, b)| acc * (b - a + 1));
    } else if id == parse_id("R") {
        return 0;
    }
    let rules = workflows.get(&id).unwrap();
    let mut ret = 0;
    for rule in rules {
        match rule {
            Rule::Lt(cat, val, dst) => {
                let mut new_inputs = inputs.clone();
                new_inputs[*cat as usize].0 = std::cmp::min(new_inputs[*cat as usize].0, val - 1);
                new_inputs[*cat as usize].1 = std::cmp::min(new_inputs[*cat as usize].1, val - 1);
                inputs[*cat as usize].0 = std::cmp::max(inputs[*cat as usize].0, *val);
                inputs[*cat as usize].1 = std::cmp::max(inputs[*cat as usize].1, *val);
                ret += traverse_rules(workflows, *dst, new_inputs);
            }
            Rule::Gt(cat, val, dst) => {
                let mut new_inputs = inputs.clone();
                new_inputs[*cat as usize].0 = std::cmp::max(new_inputs[*cat as usize].0, val + 1);
                new_inputs[*cat as usize].1 = std::cmp::max(new_inputs[*cat as usize].1, val + 1);
                inputs[*cat as usize].0 = std::cmp::min(inputs[*cat as usize].0, *val);
                inputs[*cat as usize].1 = std::cmp::min(inputs[*cat as usize].1, *val);
                ret += traverse_rules(workflows, *dst, new_inputs);
            }
            Rule::Al(dst) => {
                ret += traverse_rules(workflows, *dst, inputs);
            }
        }
    }
    ret
}

#[test_case("inputs/example-19-1.txt" => matches Ok(167409079868000))]
#[test_case("inputs/input-19.txt" => matches Ok(131899818301477))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let (workflows, _) = parse_input(filename)?;
    let ret = traverse_rules(&workflows, parse_id("in"), [(1, 4000); 4]);
    Ok(ret)
}
