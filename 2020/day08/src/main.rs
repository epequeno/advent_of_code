use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operation {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    value: isize,
    operation: Operation,
    visited: bool,
}

fn read_input(file_name: &str) -> Vec<Instruction> {
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .into_iter()
        .map(|l| match l {
            Ok(line) => {
                let split: Vec<&str> = line.split_whitespace().collect();
                let op = match split[0] {
                    "nop" => Operation::Nop,
                    "acc" => Operation::Acc,
                    "jmp" => Operation::Jmp,
                    _ => Operation::Nop,
                };
                Instruction {
                    value: split[1].parse::<isize>().unwrap(),
                    operation: op,
                    visited: false,
                }
            }
            Err(_) => Instruction {
                value: 0,
                operation: Operation::Nop,
                visited: false,
            },
        })
        .collect()
}

struct PartOneResult {
    value: isize,
    from_oob: bool,
}

fn part_one(input: &mut Vec<Instruction>) -> PartOneResult {
    let mut res = 0;
    let mut cursor = 0;
    loop {
        if cursor >= input.len() {
            return PartOneResult {
                value: res,
                from_oob: true,
            };
        }
        let current_instruction = input[cursor];
        if current_instruction.visited {
            return PartOneResult {
                value: res,
                from_oob: false,
            };
        }
        input[cursor].visited = true;
        match current_instruction.operation {
            Operation::Nop => cursor += 1,
            Operation::Acc => {
                res += current_instruction.value;
                cursor += 1;
            }
            Operation::Jmp => cursor = (cursor as isize + current_instruction.value) as usize,
        }
    }
}

fn part_two(input: &mut Vec<Instruction>) -> isize {
    for (ix, instruction) in input.iter().enumerate() {
        if instruction.operation == Operation::Acc {
            continue;
        }
        let mut test_instructions = input.clone();
        if instruction.operation == Operation::Jmp {
            test_instructions[ix].operation = Operation::Nop;
        } else {
            test_instructions[ix].operation = Operation::Jmp;
        }
        let res = part_one(&mut test_instructions);
        if res.from_oob {
            return res.value;
        }
    }
    0
}

fn main() {
    let mut input = read_input("input.txt");
    let res = part_one(&mut input);
    println!("part one: {}", res.value);

    let mut input = read_input("input.txt");
    let res = part_two(&mut input);
    println!("part two: {:?}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut test_input = read_input("test_input.txt");
        assert_eq!(part_one(&mut test_input).value, 5);
    }

    #[test]
    fn test_part_two() {
        let mut test_input = read_input("test_input.txt");
        assert_eq!(part_two(&mut test_input), 8);
    }
}
