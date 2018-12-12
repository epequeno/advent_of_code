use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Claim {
    id: u64,
    x: u64,
    y: u64,
    width: u64,
    height: u64,
}

fn parse_line(line: &str) -> Claim {
    let parts: Vec<&str> = line.split(" ").collect();
    let id = parts[0].replace("#", "").parse().unwrap();

    let x: Vec<&str> = parts[2].split(",").collect();
    let x = x[0].parse().unwrap();

    let y: Vec<&str> = parts[2].split(",").collect();
    let y = y[1].replace(":", "").parse().unwrap();

    let width: Vec<&str> = parts[3].split("x").collect();
    let width = width[0].parse().unwrap();

    let height: Vec<&str> = parts[3].split("x").collect();
    let height = height[1].parse().unwrap();

    Claim {
        id,
        x,
        y,
        width,
        height,
    }
}

fn part_one() {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    let mut board = vec![vec![0; 1000]; 1000];

    for line in f.lines() {
        let claim = parse_line(&line.unwrap());
        for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                board[x as usize][y as usize] += 1
            }
        }
    }

    let mut counter = 0;
    for row in board {
        for col in row {
            if col > 1 {
                counter += 1;
            }
        }
    }
    println!("{}", counter);
}

fn part_two() {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    let mut board = vec![vec![0; 1000]; 1000];

    for line in f.lines() {
        let claim = parse_line(&line.unwrap());
        for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                board[x as usize][y as usize] += 1
            }
        }
    }

    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    'outer: for line in f.lines() {
        let claim = parse_line(&line.unwrap());
        for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                if board[x as usize][y as usize] > 1 {
                    continue 'outer;
                }
            }
        }
        println!("{}", claim.id);
    }
}

fn main() {
    part_one();
    part_two();
}
