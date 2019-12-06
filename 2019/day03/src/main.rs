// https://adventofcode.com/2019/day/3
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_input() -> Vec<Vec<String>> {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    let mut res = Vec::new();
    for line in f.lines() {
        let line = line.unwrap();
        let instructions: Vec<String> = line.split(',').map(|s| s.to_owned()).collect();
        res.push(instructions);
    }
    res
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

fn distance(p1: Point, p2: Point) -> isize {
    (p2.x - p1.x).abs() + (p2.y - p1.y).abs()
}

fn trace_path(wire: &Vec<String>) -> Vec<Point> {
    let mut cursor = Point { x: 0, y: 0 };
    let mut res = Vec::new();

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

            res.push(cursor.clone());
        }
    }
    res
}

fn are_equal(p1: &Point, p2: &Point) -> bool {
    p1.x == p2.x && p1.y == p2.y
}

fn main() {
    let input_data = read_input();
    let wire_one = &input_data[0];
    let wire_two = &input_data[1];
    let path_one = trace_path(wire_one);
    let path_two = trace_path(wire_two);

    let mut crosses = Vec::new();
    for p1 in &path_one {
        for p2 in &path_two {
            if are_equal(p1, p2) {
                crosses.push(p1.clone());
            }
        }
    }

    let origin = Point { x: 0, y: 0 };
    let mut min_distance = distance(origin, crosses[0]);

    for point in &crosses {
        let d = distance(origin, *point);
        if d <= min_distance {
            min_distance = d;
        };
    }

    println!("part one = {}", min_distance);

    let mut min_steps = 0;
    let target_cross = crosses[0];
    for point in &path_one {
        min_steps += 1;
        if are_equal(point, &target_cross) {
            break;
        }
    }

    for point in &path_two {
        min_steps += 1;
        if are_equal(point, &target_cross) {
            break;
        }
    }

    for cross in &crosses {
        let mut total_steps = 0;
        for point in &path_one {
            total_steps += 1;
            if are_equal(point, &cross) {
                break;
            }
        }
        for point in &path_two {
            total_steps += 1;
            if are_equal(point, &cross) {
                break;
            }
        }

        if total_steps <= min_steps {
            min_steps = total_steps;
        }
    }

    println!("part two = {}", min_steps);
}
