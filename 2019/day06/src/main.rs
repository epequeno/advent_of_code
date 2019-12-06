// https://adventofcode.com/2019/day/6
use std::collections::HashMap;
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
    let mut model = HashMap::new();
    for orbit in input {
        let pair: Vec<String> = orbit.split(')').map(|s| s.to_owned()).collect();
        let key = pair[0].clone();
        let orbiter = pair[1].clone();
        let counter = model.entry(key).or_insert(0);
        *counter += 1;
    }
    println!("{:?}", model.values().sum::<usize>());
}
