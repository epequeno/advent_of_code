use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_input_part_one(file_name: &str) -> Vec<String> {
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);
    let lines: Vec<String> = f.lines().into_iter().map(|l| l.unwrap()).collect();
    let mut res = Vec::new();
    let mut answers: Vec<String> = Vec::new();
    for line in lines {
        if line.is_empty() {
            let combined = answers.join("");
            res.push(combined);
            answers = Vec::new();
            continue;
        }
        answers.push(String::from(line.trim_end()));
    }
    // since the input doesn't end on a newline we'll add the last thing we saw
    let combined = answers.join("");
    res.push(combined);
    res
}

#[derive(Debug, Clone)]
struct Group {
    // a string representation of all the unique questions that this group answered yes to.
    questions: HashSet<char>,

    // a record of how each individual person responded
    individual_responses: Vec<String>,
}

fn read_input_part_two(file_name: &str) -> Vec<Group> {
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);
    let lines: Vec<String> = f.lines().into_iter().map(|l| l.unwrap()).collect();
    let mut res = Vec::new();
    let mut questions: Vec<String> = Vec::new();
    let mut group = Group {
        questions: HashSet::new(),
        individual_responses: Vec::new(),
    };
    for line in lines {
        if !line.is_empty() {
            group.individual_responses.push(line.clone());
        }
        if line.is_empty() {
            let combined = questions.join("");
            for c in combined.chars() {
                group.questions.insert(c.clone());
            }
            res.push(group.clone());
            // reset the group
            group.questions = HashSet::new();
            group.individual_responses = Vec::new();
            questions = Vec::new();
            continue;
        }
        questions.push(String::from(line.trim_end()));
    }
    // since the input doesn't end on a newline we'll add the last thing we saw
    let combined = questions.join("");
    for c in combined.chars() {
        group.questions.insert(c.clone());
    }
    res.push(group.clone());
    res
}

fn part_one(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|line| {
            let mut set: HashSet<char> = HashSet::new();
            for c in line.chars() {
                set.insert(c.clone());
            }
            set.len()
        })
        .sum()
}

fn part_two(input: &Vec<Group>) -> usize {
    input
        .iter()
        .map(|group| {
            let mut res = 0;
            for c in group.questions.clone() {
                if group
                    .individual_responses
                    .iter()
                    .all(|response| response.contains(c))
                {
                    res += 1;
                }
            }
            res
        })
        .sum()
}

fn main() {
    let input = read_input_part_one("input.txt");
    let res = part_one(&input);
    println!("part one: {}", res);

    let input = read_input_part_two("input.txt");
    let res = part_two(&input);
    println!("part two: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = read_input_part_one("test_input.txt");
        assert_eq!(part_one(&test_input), 11);
    }

    #[test]
    fn test_part_two() {
        let test_input = read_input_part_two("test_input.txt");
        assert_eq!(part_two(&test_input), 6);
    }
}
