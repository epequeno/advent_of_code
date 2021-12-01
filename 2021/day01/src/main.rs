use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_input() -> Vec<usize> {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    f.lines()
        .into_iter()
        .map(|x| x.unwrap().parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn part_one(input: Vec<usize>) {
    let mut res: usize = 0;
    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            res += 1;
        }
    }
    println!("{}", res);
}

fn part_two(input: Vec<usize>) {
    let mut sums: Vec<usize> = Vec::new();
    for i in 0..input.len() - 1 {
        if i + 2 > input.len() - 1 {
            break;
        }
        let a = input[i];
        let b = input[i + 1];
        let c = input[i + 2];
        sums.push(a + b + c);
    }
    part_one(sums);
}

fn main() {
    let input: Vec<usize> = read_input();
    // part_one(input);
    part_two(input);
}
