use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;

fn part_one() {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    let mut has_two = 0;
    let mut has_three = 0;

    for line in f.lines() {
        let l = line.unwrap().clone();
        let l: Vec<char> = l.chars().collect();
        let set: HashSet<char> = HashSet::from_iter(l.iter().cloned()); // remove duplicates
        let mut seen_two = false;
        let mut seen_three = false;

        // if we've tried to remove duplicates andmdidn't remove anything, we don't need to
        // examine any further.
        if l.len() == set.len() {
            continue;
        }

        for c in &l {
            // we've already seen both cases we care about, we dont' need to examine any further
            if seen_two && seen_three {
                continue;
            }

            match l.iter().filter(|j| c == *j).count() {
                2 => {
                    seen_two = true;
                }
                3 => {
                    seen_three = true;
                }
                _ => {}
            }
        }

        match (seen_two, seen_three) {
            (true, true) => {
                has_two += 1;
                has_three += 1;
            }
            (true, false) => {
                has_two += 1;
            }
            (false, true) => {
                has_three += 1;
            }
            _ => {}
        }
    }

    println!("{}", has_two * has_three);
}

fn part_two() {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    let mut data = Vec::new();
    for l in f.lines() {
        data.push(l.unwrap());
    }

    for control in &data {
        let control_chars: Vec<char> = control.chars().into_iter().collect();
        for test in &data {
            let test_chars: Vec<char> = test.chars().into_iter().collect();

            let mut diffs_seen = 0;
            for i in 0..test.len() {
                if control_chars[i] != test_chars[i] {
                    diffs_seen += 1;
                }
            }
            if diffs_seen == 1 {
                let mut buf = String::new();
                for i in 0..test.len() {
                    if control_chars[i] == test_chars[i] {
                        buf += &control_chars[i].to_string();
                    }
                }
                println!("{}", buf);
            }
        }
    }
}

fn main() {
    part_one();
    part_two();
}
