use anyhow::Result;
use petgraph::prelude::*;
use std::collections::HashMap;
use test_case::test_case;

fn parse_input(filename: &str) -> Result<UnGraph<String, ()>> {
    let input = std::fs::read_to_string(filename)?;
    let mut ret = Graph::new_undirected();
    let mut nodemap = HashMap::new();
    for line in input.lines().filter(|l| !l.is_empty()) {
        let (src, dst) = line.split_once(": ").unwrap();
        let src = *nodemap.entry(src.to_string()).or_insert_with(|| ret.add_node(src.to_string()));
        for dst in dst.split(" ") {
            let dst = *nodemap.entry(dst.to_string()).or_insert_with(|| ret.add_node(dst.to_string()));
            ret.add_edge(src, dst, ());
        }
    }
    return Ok(ret);
}

#[test_case("inputs/example-25-1.txt" => matches Ok(54))]
#[test_case("inputs/input-25.txt" => matches Ok(562912))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let mut input = parse_input(filename)?;
    // let dot = petgraph::dot::Dot::with_config(&input, &[petgraph::dot::Config::EdgeNoLabel]);
    // std::fs::write("day25.dot", format!("{:?}", dot))?;
    for _ in 0..3 {
        let mut counts = HashMap::new();
        for i in fastrand::choose_multiple(input.node_indices(), 12) {
            for j in fastrand::choose_multiple(input.node_indices(), 12) {
                let (_, path) = petgraph::algo::astar::astar(&input, i, |n| n == j, |_| 1, |_| 1).unwrap();
                let mut prev = i;
                for n in path {
                    if prev != i {
                        *counts.entry((prev, n)).or_insert(0) += 1;
                    }
                    prev = n;
                }
            }
        }
        let mut top = counts.iter().collect::<Vec<_>>();
        top.sort_by_key(|(_, v)| **v);
        let last_edge = top.last().unwrap().0;
        let last_edge = input.find_edge(last_edge.0, last_edge.1).unwrap();
        input.remove_edge(last_edge);
    }
    let ret = petgraph::algo::tarjan_scc(&input).iter().map(|c| c.len()).product::<usize>() as i64;
    Ok(ret)
}
