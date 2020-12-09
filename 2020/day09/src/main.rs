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

fn part_one(input: &Vec<usize>, preamble_len: usize) -> Option<usize> {
    for i in 0..input.len() - preamble_len {
        let end = i + preamble_len;
        if end == input.len() {
            break;
        }
        let target_value = &input[end];
        let preamble = &input[i..end];
        if !preamble.iter().any(|x| {
            if let Some(res) = target_value.checked_sub(*x) {
                preamble.contains(&res)
            } else {
                false
            }
        }) {
            return Some(*target_value);
        }
    }
    None
}

fn part_two(input: &Vec<usize>, target_value: usize) -> usize {
    for i in 0..input.len() {
        if input[i] >= target_value {
            continue;
        }
        let start = input[i];
        let mut acc = input[i];
        let rest = &input[i + 1..];
        'inner: for j in 0..rest.len() {
            acc += rest[j];
            if acc > target_value {
                break 'inner;
            }
            if acc == target_value {
                let mut group = vec![start];
                group.extend(&rest[..=j]);
                let mut max = 0;
                let mut min = start;
                for k in &group {
                    if k > &max {
                        max = *k;
                    }
                    if k < &min {
                        min = *k;
                    }
                }
                return min + max;
            }
        }
    }
    0
}

fn main() {
    let input = read_input("input.txt");
    let res = part_one(&input, 25);
    println!("part one: {}", res.unwrap());

    let res = part_two(&input, res.unwrap());
    println!("part two: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = read_input("test_input.txt");
        assert_eq!(part_one(&test_input, 5).unwrap(), 127);
    }

    #[test]
    fn test_part_two() {
        let test_input = read_input("test_input.txt");
        let target_value = part_one(&test_input, 5);
        assert_eq!(part_two(&test_input, target_value.unwrap()), 62);
    }
}
