// https://adventofcode.com/2019/day/2
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_input() -> Vec<usize> {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    let mut res = Vec::new();
    for line in f.lines() {
        let line = line.unwrap();
        let nums: Vec<&str> = line.split(',').collect();
        for i in nums.iter() {
            res.push(i.parse::<usize>().unwrap())
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
    let data = read_input();

    // part one
    println!("part 1 = {:?}", intcode_computer(data.clone(), 12, 2));

    // part two
    for noun in 0..99 {
        for verb in 0..99 {
            if intcode_computer(data.clone(), noun, verb) == 19690720 {
                println!("part 2 = {}", 100 * noun + verb);
                return;
            }
        }
    }
}
