// https://adventofcode.com/2019/day/3
// optimization ideas from:
//   https://old.reddit.com/r/adventofcode/comments/e5bz2w/2019_day_3_solutions/f9iz68s/
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn read_input() -> Vec<Vec<String>> {
    let input = read_to_string("input.txt").unwrap();
    let mut res = Vec::new();
    for line in input.lines() {
        let instructions: Vec<String> = line.split(',').map(|s| s.to_owned()).collect();
        res.push(instructions);
    }
    res
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

fn distance(p1: Point, p2: Point) -> isize {
    (p2.x - p1.x).abs() + (p2.y - p1.y).abs()
}

fn trace_path(wire: &Vec<String>) -> HashMap<Point, isize> {
    let mut cursor = Point { x: 0, y: 0 };
    let mut res = HashMap::new();
    let mut step_counter = 0;

    for movement in wire {
        let direction = movement.chars().next().unwrap();
        let steps: isize = movement[1..].parse().unwrap();

        for _ in 0..steps {
            match direction {
                'U' => {
                    cursor.y += 1;
                }
                'D' => {
                    cursor.y -= 1;
                }
                'R' => {
                    cursor.x += 1;
                }
                'L' => {
                    cursor.x -= 1;
                }
                _ => {
                    println!("unknown char found! {}", direction);
                }
            }

            // keep track of how many steps it took us to get to this point
            // only record the first time we've visited the point
            step_counter += 1;
            if !res.contains_key(&cursor) {
                res.insert(cursor.clone(), step_counter);
            }
        }
    }
    res
}

fn main() {
    let input_data = read_input();
    let wire_one = &input_data[0];
    let wire_two = &input_data[1];

    let path_one = trace_path(wire_one);
    let path_two = trace_path(wire_two);

    let mut path_one_points = HashSet::new();
    let mut path_two_points = HashSet::new();

    for point in path_one.keys() {
        path_one_points.insert(point);
    }

    for point in path_two.keys() {
        path_two_points.insert(point);
    }

    let origin = Point { x: 0, y: 0 };
    let crosses: Vec<Point> = path_one_points
        .intersection(&path_two_points)
        .map(|p| p.to_owned())
        .map(|p| p.to_owned())
        .collect();
    let res = crosses.iter().map(|p| distance(origin, *p)).min().unwrap();
    println!("part one = {}", res);

    let res = crosses
        .iter()
        .map(|p| path_one[p] + path_two[p])
        .min()
        .unwrap();
    println!("part two = {}", res);
}
