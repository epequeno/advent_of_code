use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
enum Direction {
    Forward(usize),
    Up(usize),
    Down(usize),
}

fn read_input() -> Vec<Direction> {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    f.lines()
        .into_iter()
        .map(|x| {
            let instruction = x.unwrap();
            let direction: Vec<&str> = instruction.split_whitespace().collect();
            let units: usize = direction[1].parse().unwrap();
            match direction[0] {
                "forward" => Direction::Forward(units),
                "up" => Direction::Up(units),
                "down" => Direction::Down(units),
                _ => Direction::Forward(0),
            }
        })
        .collect()
}

fn part_one(input: &Vec<Direction>) -> usize {
    let mut horizontal: usize = 0;
    let mut depth: usize = 0;
    for d in input {
        match d {
            Direction::Forward(n) => horizontal += n,
            Direction::Up(n) => depth -= n,
            Direction::Down(n) => depth += n,
        }
    }
    horizontal * depth
}

fn part_two(input: &Vec<Direction>) -> usize {
    let mut horizontal: usize = 0;
    let mut depth: usize = 0;
    let mut aim: usize = 0;
    for d in input {
        match d {
            Direction::Forward(n) => {
                horizontal += n;
                depth += aim * n;
            }
            Direction::Up(n) => aim -= n,
            Direction::Down(n) => aim += n,
        }
    }
    horizontal * depth
}

fn main() {
    let input = read_input();
    let res = part_one(&input);
    println!("part 1: {}", res);

    let res = part_two(&input);
    println!("part 2: {}", res);
}
