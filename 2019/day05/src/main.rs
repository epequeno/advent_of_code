// https://adventofcode.com/2019/day/5
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_input() -> Vec<isize> {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    let mut res = Vec::new();
    for line in f.lines() {
        let line = line.unwrap();
        let nums: Vec<&str> = line.split(',').collect();
        for i in nums.iter() {
            res.push(i.parse::<isize>().unwrap());
        }
    }
    res
}

fn intcode_computer(mut data: Vec<usize>, noun: usize, verb: usize) -> usize {
    // split data into opcode groups
    let mut opcodes = Vec::new();
    let cloned_data = data.clone();
    for chunk in cloned_data.chunks(4) {
        if chunk.len() < 4 {
            continue;
        }
        opcodes.push(chunk);
    }

    // set values according to puzzle instructions
    data[1] = noun;
    data[2] = verb;

    for opcode in opcodes.iter() {
        match opcode[0] {
            1 => {
                data[opcode[3]] = data[opcode[1]] + data[opcode[2]];
            }
            2 => {
                data[opcode[3]] = data[opcode[1]] * data[opcode[2]];
            }
            _ => {}
        }
    }

    data[0]
}

fn main() {
    let input = read_input();
    println!("{:?}", input);
}
