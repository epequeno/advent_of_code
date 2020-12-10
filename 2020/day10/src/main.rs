use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_input(file_name: &str) -> Vec<usize> {
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .into_iter()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
}

fn part_one(input: &Vec<usize>) -> usize {
    let mut cursor = 0;
    let mut diffs: Vec<usize> = Vec::new();
    for _ in 0..=input.len() {
        'inner: for i in 1..=3 {
            if input.contains(&(cursor + i)) {
                diffs.push(i);
                cursor += i;
                // we want to reset the counter when we find a match so we can start searching from
                // 1 again.
                break 'inner;
            }
        }
    }
    // since your adpater is _always_ a 3-diff above the max
    diffs.push(3);
    let one_diffs = diffs.iter().filter(|&d| *d == 1).count();
    let three_diffs = diffs.iter().filter(|&d| *d == 3).count();
    one_diffs * three_diffs
}

// from: https://github.com/jchevertonwynne/advent_of_code_2020/blob/3a307d2ca38190d85ad4f9599d703d28e6468ab1/src/days/day10.rs#L29
fn part_two(mut input: Vec<usize>) -> usize {
    let after_zero = input.iter().take_while(|&j| *j <= 3).count();

    (0..input.len()).for_each(|i| {
        input[i] = input[i + 1..]
            .iter()
            .take_while(|j| *j - input[i] <= 3)
            .count();
    });
    let ind = input.len() - 1;
    input[ind] = 1;

    (0..input.len() - 1)
        .rev()
        .for_each(|i| input[i] = input[i + 1..i + 1 + input[i]].iter().sum());

    input[..after_zero].iter().sum()
}

fn main() {
    // let input = read_input("input.txt");
    // let res = part_one(&input);
    // println!("part one: {}", res);

    let mut input = read_input("input.txt");
    input.sort();
    let res = part_two(input);
    println!("part two: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = read_input("test_input_part_one.txt");
        assert_eq!(part_one(&test_input), 220);
    }

    #[test]
    fn test_part_two() {
        let mut test_input = read_input("test_input_part_two.txt");
        test_input.sort();
        assert_eq!(part_two(test_input), 8);
    }
}
