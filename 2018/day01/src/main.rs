use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn parse_line(line: &str) -> isize {
    line[1..].parse().unwrap()
}

fn part_one() {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);

    let mut result = 0;

    for line in f.lines() {
        let l = line.unwrap();
        if l.starts_with("+") {
            result += parse_line(&l);
        } else {
            result -= parse_line(&l);
        }
    }
    println!("{}", result);
}

fn part_two() {
    let mut f = File::open("input.txt").unwrap();
    let mut result = 0;
    let mut history = HashSet::new();

    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let lines: Vec<&str> = s.split_whitespace().collect();

    for l in lines.into_iter().cycle() {
        if l.starts_with("+") {
            result += parse_line(&l);
        } else {
            result -= parse_line(&l);
        }

        if history.contains(&result) {
            println!("{}", result);
            break;
        }

        history.insert(result);
    }
}

fn main() {
    part_one();
    part_two();
}
