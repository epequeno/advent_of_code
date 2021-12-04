use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_input() -> Vec<Vec<usize>> {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    f.lines()
        .into_iter()
        .map(|x| {
            x.unwrap()
                .chars()
                .into_iter()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn part_one(input: &Vec<Vec<usize>>) -> usize {
    let mut gamma: Vec<String> = Vec::new();
    let mut epsilon: Vec<String> = Vec::new();
    for i in 0..input[0].len() {
        let mut group = Vec::new();
        for line in input {
            group.push(line[i]);
        }
        if group.iter().sum::<usize>() > (group.len() / 2) {
            // more 1 than 0
            gamma.push("1".into());
            epsilon.push("0".into())
        } else {
            gamma.push("0".into());
            epsilon.push("1".into())
        }
    }
    // vec of usize -> binary string
    let g = gamma.join("");
    let e = epsilon.join("");
    usize::from_str_radix(&g, 2).unwrap() * usize::from_str_radix(&e, 2).unwrap()
}

fn part_two(input: &Vec<Vec<usize>>) -> usize {
    filter(input, true) * filter(input, false)
}

fn filter(data: &Vec<Vec<usize>>, is_oxy: bool) -> usize {
    let mut candidates = data.clone();
    let mut ix = 0;
    loop {
        if candidates.len() == 1 {
            break;
        }
        let group: Vec<usize> = candidates.clone().into_iter().map(|c| c[ix]).collect();
        let zeros = group
            .iter()
            .filter(|n| **n == 0)
            .collect::<Vec<&usize>>()
            .len();
        let ones = group
            .iter()
            .filter(|n| **n == 1)
            .collect::<Vec<&usize>>()
            .len();
        let target = if zeros > ones {
            if is_oxy {
                0
            } else {
                1
            }
        } else if ones > zeros {
            if is_oxy {
                1
            } else {
                0
            }
        } else {
            if is_oxy {
                1
            } else {
                0
            }
        };
        candidates = candidates
            .clone()
            .into_iter()
            .filter(|c| c[ix] == target)
            .to_owned()
            .collect();
        ix += 1;
    }
    let s = candidates[0]
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("");
    usize::from_str_radix(&s, 2).unwrap()
}

fn main() {
    let input = read_input();
    println!("part 1: {:?}", part_one(&input));
    println!("part 2: {:?}", part_two(&input));
}
