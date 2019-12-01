// https://adventofcode.com/2019/day/1
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn rounder(n: &f64) -> f64 {
    ((n) / 3.).floor() - 2.
}

fn read_input() -> Vec<f64> {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    let mut res = Vec::new();
    for line in f.lines() {
        let num = line.unwrap().parse::<f64>().unwrap();
        res.push(num);
    }
    res
}

fn calculate_additional_fuel(n: &f64) -> f64 {
    let mut total: f64 = 0.;
    let mut res = rounder(n);
    while res >= 0. {
        total += res;
        res = rounder(&res);
    }
    total
}

fn main() {
    let examples: Vec<f64> = vec![12., 14., 1969., 100756.];
    let _res: f64 = examples.iter().map(|x| rounder(x)).sum();

    let data = read_input();

    // part one
    let res: f64 = data.iter().map(|x| rounder(x)).sum();
    println!("part 1 = {}", res);

    // part two
    let res: f64 = data.iter().map(|n| calculate_additional_fuel(n)).sum();
    println!("part 2 = {}", res);
}
