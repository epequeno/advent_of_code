use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;

fn read_input() -> Vec<usize> {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    f.lines()
        .into_iter()
        .map(|x| x.unwrap().parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn part_one() {
    let input: Vec<usize> = read_input();
    let input_set: HashSet<usize> = HashSet::from_iter(input.iter().cloned());
    for i in input {
        let target: usize = 2020 - i;
        if input_set.contains(&target) {
            println!("{}", i * target);
            // we expect to find a matching value twice (A*B and later B*A), we can stop looking
            // after the first match.
            break;
        }
    }
}

fn part_two() {
    let input: Vec<usize> = read_input();
    'outer: for i in &input {
        for j in &input {
            for k in &input {
                if i + j + k == 2020 {
                    println!("{}", i * j * k);
                    break 'outer;
                }
            }
        }
    }
}

fn main() {
    // part_one();
    part_two();
}
