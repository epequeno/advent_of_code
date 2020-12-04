use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_input(file_name: &str) -> String {
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);
    let lines: Vec<String> = f.lines().into_iter().map(|l| l.unwrap()).collect();
    lines[0].clone()
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn step(&mut self, direction: char) {
        match direction {
            '^' => self.y += 1,
            'v' => self.y -= 1,
            '<' => self.x -= 1,
            '>' => self.x += 1,
            _ => (),
        }
    }
}

fn part_one(input: &str) -> usize {
    let mut seen: HashSet<Point> = HashSet::new();
    let mut cursor = Point { x: 0, y: 0 };
    seen.insert(cursor);
    for c in input.chars() {
        cursor.step(c);
        seen.insert(cursor.clone());
    }
    seen.len()
}

fn part_two(input: &str) -> usize {
    let mut santa_seen: HashSet<Point> = HashSet::new();
    let mut santa_cursor = Point { x: 0, y: 0 };
    santa_seen.insert(santa_cursor);

    let mut robo_santa_seen: HashSet<Point> = HashSet::new();
    let mut robo_santa_cursor = Point { x: 0, y: 0 };
    robo_santa_seen.insert(robo_santa_cursor);

    for c in input.chars().step_by(2) {
        santa_cursor.step(c);
        santa_seen.insert(santa_cursor.clone());
    }
    for c in input.chars().skip(1).step_by(2) {
        robo_santa_cursor.step(c);
        robo_santa_seen.insert(robo_santa_cursor.clone());
    }
    let intersection: Vec<&Point> = santa_seen.union(&robo_santa_seen).collect();
    intersection.len()
}

fn main() {
    let input = read_input("input.txt");
    println!("part one: {}", part_one(&input));

    println!("part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_case_1() {
        assert_eq!(part_one(&">"), 2);
    }

    #[test]
    fn test_part_one_case_2() {
        assert_eq!(part_one(&"^>v<"), 4);
    }

    #[test]
    fn test_part_one_case_3() {
        assert_eq!(part_one(&"^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_part_two_case_1() {
        assert_eq!(part_two(&"^v"), 3);
    }

    #[test]
    fn test_part_two_case_2() {
        assert_eq!(part_two(&"^>v<"), 3);
    }

    #[test]
    fn test_part_two_case_3() {
        assert_eq!(part_two(&"^v^v^v^v^v"), 11);
    }
}
