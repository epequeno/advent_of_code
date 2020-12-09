// basically just a re-implementation of
// https://github.com/dmend/advent-of-code/blob/c9a821cce0fbed29e43b6b2fdf47a074b33dfbb7/2020/day07/day07.py
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn parse_contents(input: &str) -> Vec<String> {
    if input.contains("no other bags") {
        Vec::new()
    } else {
        let split: Vec<&str> = input.split(",").collect();
        split
            .iter()
            .map(|content| {
                let bag: Vec<&str> = content.split_whitespace().collect();
                bag[1..3].join("_")
            })
            .collect()
    }
}

fn read_input(file_name: &str) -> HashMap<String, Vec<String>> {
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);
    let mut res = HashMap::new();
    let _ = f.lines().into_iter().for_each(|line| {
        let line = line.unwrap();
        let split: Vec<&str> = line.split("contain").collect();
        let outer = split[0]
            .trim_end()
            .replace("bags", "")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("_");
        res.insert(outer, parse_contents(split[1]));
    });
    res
}

fn find(target_bag: &str, in_bag: &str, bag_map: HashMap<String, Vec<String>>) -> bool {
    if !bag_map.contains_key(in_bag) {
        false
    } else {
        bag_map[in_bag].contains(&target_bag.to_string())
            || bag_map[in_bag]
                .iter()
                .any(|b| find(target_bag, b, bag_map.clone()))
    }
}

fn part_one(all_bags: HashMap<String, Vec<String>>) -> usize {
    all_bags
        .keys()
        .map(|k| {
            if find("shiny_gold", &(k.clone()), all_bags.clone()) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let input = read_input("input.txt");
    let res = part_one(input);
    println!("{}", res);
}
