use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_point(point: &str) -> Point {
    let parts: Vec<&str> = point.split(",").collect();
    Point {
        x: parts[0].parse().unwrap(),
        y: parts[1].trim().parse().unwrap(),
    }
}

fn find_bounds(points: &Vec<Point>) -> (Point, Point) {
    let mut min_x = 1000;
    let mut max_x = 0;

    let mut min_y = 1000;
    let mut max_y = 0;

    for point in points {
        if point.x < min_x {
            min_x = point.x;
        }

        if point.y < min_y {
            min_y = point.y;
        }

        if point.x > max_x {
            max_x = point.x;
        }

        if point.y > max_y {
            max_y = point.y;
        }
    }
    let lower = Point { x: min_x, y: min_y };
    let upper = Point { x: max_x, y: max_y };
    (lower, upper)
}

/// https://en.wikipedia.org/wiki/Taxicab_geometry#Formal_definition
fn distance(p1: Point, p2: Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn part_one() {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    let mut points = Vec::new();

    for line in f.lines() {
        points.push(parse_point(&line.unwrap()));
    }

    // get original set of bounds
    let (lower_bound, _upper_bound) = find_bounds(&points);

    // shift the points to orient around orgin rather than in the middle of the 1st quadrant
    for mut point in &mut points {
        point.x = point.x - lower_bound.x;
        point.y = point.y - lower_bound.y;
    }

    // get new set of bounds
    let (_lower_bound, upper_bound) = find_bounds(&points);

    let mut board = vec![vec![0_i64; upper_bound.x as usize]; upper_bound.y as usize];
    for y in 0..upper_bound.y {
        for x in 0..upper_bound.x {
            let current_point = Point { x, y };
            let current_value = board[y as usize][x as usize];
            'inner: for point in &points {
                let distance = distance(current_point, *point);

                // our current spot on the board is one of the points, mark it
                if distance == 0 {
                    board[y as usize][x as usize] = -2;
                    continue 'inner;
                }

                // these values have special significance and if seen should not be changed.
                if current_value == -1 || current_value == -2 {
                    continue 'inner;
                }

                if current_value == distance.into() {
                    board[y as usize][x as usize] = -1;
                } else if (current_value == 0) || (current_value > distance.into()) {
                    board[y as usize][x as usize] = distance.into();
                }
            }
        }
    }
    println!("{:?}", board);
}

fn main() {
    part_one();
}
