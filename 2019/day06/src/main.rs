// https://adventofcode.com/2019/day/6
use petgraph::Graph;
use std::collections::HashSet;
use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    let input = read_to_string("input.txt").unwrap();
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(line.to_owned());
    }
    result
}

fn main() {
    let input = read_input();
    let mut deps = Graph::<String, String>::new();
    let mut nodes = HashSet::new();
    for orbit in input.clone() {
        let pair: Vec<String> = orbit.split(')').map(|s| s.to_owned()).collect();
        nodes.insert(pair[0].clone());
        nodes.insert(pair[1].clone());
    }

    for node in nodes {
        deps.add_node(node);
    }


    for orbit in input {
        let edge = 
    }
}
