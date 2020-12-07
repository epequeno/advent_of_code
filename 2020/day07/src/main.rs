// WIP
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bag {
    descriptor: String,
    color: String,
    contents: Vec<Bag>,
    can_contain_shiny_gold: bool,
}

fn read_input(file_name: &str) -> Vec<Bag> {
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .into_iter()
        .map(|l| match l {
            Ok(line) => {
                let split: Vec<&str> = line.split("contain").collect();
                let bag_descriptor = split[0].replace("bags", "");
                let bag_descriptor: Vec<&str> = bag_descriptor.split_whitespace().collect();
                let mut all_contents = Vec::new();
                if !split[1].contains("no other bags") {
                    let contents: Vec<&str> = split[1].split(",").collect();
                    for c in contents {
                        let res = parse_content_descriptor(c);
                        for b in res {
                            all_contents.push(b);
                        }
                    }
                }
                Bag {
                    descriptor: bag_descriptor[0].into(),
                    color: bag_descriptor[1].into(),
                    contents: all_contents,
                    can_contain_shiny_gold: false,
                }
            }
            Err(_) => panic!(),
        })
        .collect()
}

fn parse_content_descriptor(description: &str) -> Vec<Bag> {
    let mut res = Vec::new();
    let split: Vec<&str> = description.trim_start().split(' ').collect();
    let count = split[0].parse::<usize>().unwrap();
    for _ in 0..count {
        let bag = Bag {
            descriptor: split[1].into(),
            color: split[2].into(),
            contents: Vec::new(),
            can_contain_shiny_gold: false,
        };
        res.push(bag);
    }
    res
}

fn find_direct(bags: &Vec<Bag>) -> Vec<Bag> {
    let mut res = Vec::new();
    for bag in bags {
        'inner: for c in &bag.contents {
            if c.descriptor == "shiny" && c.color == "gold" {
                let mut new_bag = bag.clone();
                new_bag.can_contain_shiny_gold = true;
                res.push(new_bag);
                break 'inner;
            }
        }
    }
    res
}

fn count_children(bags: Vec<Bag>) -> usize {
    let possible: Vec<Bag> = bags
        .iter()
        .filter(|b| b.can_contain_shiny_gold)
        .cloned()
        .collect();
    if possible.is_empty() {
        return 0;
    } else {
        return 1 + possible[1..]
            .iter()
            .map(|b| count_children(b.contents.clone()))
            .sum::<usize>();
    }
}

fn part_one(input: &Vec<Bag>) -> usize {
    let mut res = 0;
    let direct_bags = find_direct(&input);
    let mut new_bags: Vec<Bag> = Vec::new();
    for bag in input {
        let mut new_bag = bag.clone();
        new_bag.contents = Vec::new();
        for c in &bag.contents {
            for db in &direct_bags {
                if c.descriptor == db.descriptor && c.color == db.color {
                    let mut new_content = c.clone();
                    new_content.can_contain_shiny_gold = true;
                    new_bag.contents.push(new_content);
                }
            }
        }
        new_bags.push(new_bag);
    }

    for bag in new_bags {
        if !bag.contents.is_empty() {
            if count_children(bag.contents.clone()) >= 1 {
                res += 1;
            }
        }
        for db in &direct_bags {
            if bag.descriptor == db.descriptor && bag.color == db.color {
                res += 1;
            }
        }
    }
    res
}

fn main() {
    let mut input = read_input("input.txt");
    let res = part_one(&mut input);
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut test_input = read_input("test_input.txt");
        assert_eq!(part_one(&mut test_input), 4);
    }
}
