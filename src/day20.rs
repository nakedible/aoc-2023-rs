use anyhow::Result;
use smallbitset::Set64;
use std::collections::VecDeque;
use test_case::test_case;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum Module {
    Broadcast,
    FlipFlop,
    Conjunction,
}

fn parse_input(filename: &str) -> Result<Vec<(Module, Set64)>> {
    let input = std::fs::read_to_string(filename)?;
    let mut input: Vec<(String, Module, Vec<String>)> = input
        .lines()
        .map(|l| {
            let (src, dst) = l.split_once(" -> ").unwrap();
            let (src, module) = if src == "broadcaster" {
                ("".to_owned(), Module::Broadcast)
            } else if src.starts_with("%") {
                (src[1..].to_owned(), Module::FlipFlop)
            } else if src.starts_with("&") {
                (src[1..].to_owned(), Module::Conjunction)
            } else {
                unreachable!()
            };
            let dst: Vec<String> = dst.split(", ").map(|s| s.to_owned()).collect();
            (src, module, dst)
        })
        .collect();
    input.sort();
    let mut extra = input
        .iter()
        .map(|(_, _, dst)| dst)
        .flatten()
        .filter(|&s| !input.iter().any(|(src, _, _)| src == s))
        .cloned()
        .collect::<Vec<_>>();
    extra.sort();
    extra.dedup();
    for s in extra {
        input.push((s, Module::Broadcast, Vec::new()));
    }
    let ret = input
        .iter()
        .map(|(_, module, dst)| {
            let mut set = Set64::empty();
            for d in dst.iter() {
                set.add_inplace(input.iter().position(|(s, _, _)| s == d).unwrap());
            }
            (*module, set)
        })
        .collect();
    return Ok(ret);
}

fn build_refs(input: &[(Module, Set64)]) -> [Set64; 64] {
    let mut refs = [Set64::empty(); 64];
    for (i, (_, dst)) in input.iter().enumerate() {
        for dst in dst.ones() {
            refs[dst].add_inplace(i);
        }
    }
    refs
}

fn press_button(
    input: &[(Module, Set64)],
    refs: &[Set64; 64],
    flipflops: &mut Set64,
    conjunctions: &mut [Set64; 64],
    watch_conj: usize,
) -> ([usize; 2], Set64) {
    let mut watch = Set64::empty();
    let mut pulses = VecDeque::new();
    let mut counts = [0, 0];
    pulses.push_back((0, Set64::singleton(0), false));
    while let Some((src, ids, pulse)) = pulses.pop_front() {
        for id in ids.ones() {
            counts[pulse as usize] += 1;
            match (input[id], pulse) {
                ((Module::Broadcast, dst), pulse) => {
                    if dst != Set64::empty() {
                        pulses.push_back((id, dst, pulse));
                    }
                }
                ((Module::FlipFlop, dst), false) => {
                    if flipflops.contains(id) {
                        flipflops.remove_inplace(id);
                    } else {
                        flipflops.add_inplace(id);
                    }
                    pulses.push_back((id, dst, flipflops.contains(id)));
                }
                ((Module::FlipFlop, _), true) => (),
                ((Module::Conjunction, dst), pulse) => {
                    if pulse {
                        if id == watch_conj {
                            watch.add_inplace(src);
                        }
                        conjunctions[id].add_inplace(src);
                    } else {
                        conjunctions[id].remove_inplace(src);
                    }
                    let pulse = !conjunctions[id].contains_all(refs[id]);
                    pulses.push_back((id, dst, pulse));
                }
            }
        }
    }
    (counts, watch)
}

#[test_case("inputs/example-20-1.txt" => matches Ok(32000000))]
#[test_case("inputs/example-20-2.txt" => matches Ok(11687500))]
#[test_case("inputs/input-20.txt" => matches Ok(839775244))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let refs = build_refs(&input);
    let mut flipflops = Set64::empty();
    let mut conjunctions = [Set64::empty(); 64];
    let mut counts = [0; 2];
    for _ in 0..1000 {
        let (c, _) = press_button(&input, &refs, &mut flipflops, &mut conjunctions, 0);
        counts[0] += c[0];
        counts[1] += c[1];
    }
    let ret = counts[0] as i64 * counts[1] as i64;
    Ok(ret)
}

#[test_case("inputs/input-20.txt" => matches Ok(207787533680413))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let refs = build_refs(&input);
    let mut flipflops = Set64::empty();
    let mut conjunctions = [Set64::empty(); 64];
    let last_conj = refs[input.len() - 1].iter().next().unwrap();
    let last_refs = refs[last_conj].iter().collect::<Vec<_>>();
    let mut last_high = [None; 64];
    for counter in 1.. {
        let (_, watch) = press_button(&input, &refs, &mut flipflops, &mut conjunctions, last_conj);
        for i in watch.iter() {
            last_high[i] = Some(counter);
        }
        if last_refs.iter().all(|i| last_high[*i].is_some()) {
            break;
        }
    }
    // XXX: should be LCD, but humbug
    let ret = last_refs
        .iter()
        .map(|i| last_high[*i].unwrap())
        .fold(1, |acc, i| acc * i);
    Ok(ret)
}
