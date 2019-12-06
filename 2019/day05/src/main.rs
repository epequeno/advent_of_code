// https://adventofcode.com/2019/day/5
use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    let input = read_to_string("input.txt").unwrap();
    let mut res = Vec::new();
    for line in input.lines() {
        let ops: Vec<String> = line.split(',').map(|s| s.to_owned()).collect();
        for op in ops.iter() {
            res.push(op.to_owned());
        }
    }
    res
}

enum ParameterMode {
    Position,
    Immediate,
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
            3 => {}
            4 => {}
            _ => {}
        }
    }

    data[0]
}

fn main() {
    let input = read_input();
    println!("{:?}", input);
}
