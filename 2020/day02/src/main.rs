use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct PasswordConfig {
    min: usize,
    max: usize,
    target: char,
    password: String,
}

impl PasswordConfig {
    #[allow(dead_code)]
    fn is_valid_part_one(&self) -> bool {
        let match_count = self.password.matches(self.target).count();
        (self.min <= match_count) && (match_count <= self.max)
    }

    // part two changed the rules so really min and max aren't the number of char occurances we're
    // looking for, it's the index of the first and second chars to check. Rather than rename the
    // struct fields we'll just run with them.
    // Note: Be careful; Toboggan Corporate Policies have no concept of "index zero"!
    // that means we'll have to offset our values by one to index the first item as `1`
    fn is_valid_part_two(&self) -> bool {
        let first_char = self.password.chars().nth(self.min - 1).unwrap();
        let second_char = self.password.chars().nth(self.max - 1).unwrap();
        !(self.target == first_char && self.target == second_char)
            && (self.target == first_char || self.target == second_char)
    }
}

fn read_input() -> Vec<PasswordConfig> {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    f.lines()
        .into_iter()
        .map(|line| {
            let line = line.unwrap();
            let split: Vec<&str> = line.split(':').collect();
            let password: String = split[1].split_whitespace().collect();

            let split: Vec<&str> = split[0].split(' ').collect();
            let target: char = split[1]
                .split_whitespace()
                .collect::<String>()
                .parse()
                .unwrap();

            let split: Vec<&str> = split[0].split('-').collect();
            let min: usize = split[0].parse().unwrap();
            let max: usize = split[1].parse().unwrap();

            PasswordConfig {
                min,
                max,
                target,
                password,
            }
        })
        .collect()
}

fn main() {
    let result = read_input()
        .iter()
        .filter(|x| x.is_valid_part_two())
        .count();
    println!("{}", result);
}
