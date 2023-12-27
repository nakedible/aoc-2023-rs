use anyhow::Result;
use std::collections::HashMap;
use test_case::test_case;

type Node = [u8; 3];

fn parse_input(filename: &str) -> Result<(Vec<bool>, HashMap<Node, (Node, Node)>)> {
    let input = std::fs::read_to_string(filename)?;
    let (dirs, nodes) = input.split_once("\n\n").unwrap();
    let dirs: Vec<bool> = dirs
        .chars()
        .map(|c| match c {
            'L' => false,
            'R' => true,
            _ => panic!("invalid direction"),
        })
        .collect();
    let nodes: HashMap<Node, (Node, Node)> = nodes
        .lines()
        .map(|line| {
            let (node, branch) = line.split_once(" = ").unwrap();
            let node: Node = node.as_bytes().try_into().unwrap();
            let (left, right) = branch.split_once(", ").unwrap();
            let left: Node = left.trim_start_matches('(').as_bytes().try_into().unwrap();
            let right: Node = right.trim_end_matches(')').as_bytes().try_into().unwrap();
            (node, (left, right))
        })
        .collect();
    Ok((dirs, nodes))
}

#[test_case("inputs/example-08-1.txt" => matches Ok(2))]
#[test_case("inputs/example-08-2.txt" => matches Ok(6))]
#[test_case("inputs/input-08.txt" => matches Ok(18113))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let (dirs, nodes) = input;
    let mut node = [b'A', b'A', b'A'];
    let mut count = 0;
    for &r in dirs.iter().cycle() {
        let (left, right) = nodes[&node];
        node = if r { right } else { left };
        count += 1;
        if node == [b'Z', b'Z', b'Z'] {
            break;
        }
    }
    Ok(count)
}

#[test_case("inputs/example-08-3.txt" => matches Ok(6))]
#[test_case("inputs/input-08.txt" => matches Ok(12315788159977))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let (dirs, nodes) = input;
    let mut cur: Vec<Node> = nodes.keys().filter(|k| k[2] == b'A').cloned().collect();
    let mut count = 0;
    let mut lens = Vec::new();
    for &r in dirs.iter().cycle() {
        for n in cur.iter_mut() {
            let (left, right) = nodes[n];
            *n = if r { right } else { left };
        }
        count += 1;
        cur.retain(|c| {
            if c[2] == b'Z' {
                lens.push(count);
                false
            } else {
                true
            }
        });
        if cur.is_empty() {
            break;
        }
    }
    println!("{:?}", lens);
    let lcm = lens.into_iter().fold(1, num::integer::lcm);
    Ok(lcm)
}
