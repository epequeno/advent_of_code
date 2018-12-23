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

fn mark_cell(p: Point, points: &Vec<Point>) -> String {
    let mut min = 10000;
    let mut ans = String::new();
    for point in points {
        let distance = distance(p, *point);
        let repr = format!("({},{})", point.x, point.y);
        if distance == 0 {
            return repr;
        }

        if distance < min {
            min = distance;
            ans = repr;
        } else if distance == min {
            return ".".to_string();
        }
    }
    ans
}

fn part_one() {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    let mut points = Vec::new();

    for line in f.lines() {
        points.push(parse_point(&line.unwrap()));
    }

    // get original set of bounds
    let (lower_bound, _) = find_bounds(&points);

    // TODO: the following "shifting" may correctly identify the bounded points but might also be
    // throwing off the count later on.

    // shift points to start at 0 instead of somewhere in the middle of the 1st quadrant.
    // this allows for a more direct model when mapping between points and array indices
    let shifted = points
        .iter()
        .map(|p| {
            let x = p.x - lower_bound.x;
            let y = p.y - lower_bound.y;
            Point { x, y }
        })
        .collect();

    // get new set of bounds
    let (_, upper_bound) = find_bounds(&shifted);

    let mut board =
        vec![vec![String::new(); (upper_bound.y + 1) as usize]; (upper_bound.x + 1) as usize];

    // fill out the board.
    for x in 0..board.len() {
        for y in 0..board[x].len() {
            let p = Point {
                x: x as i32,
                y: y as i32,
            };
            board[x][y] = mark_cell(p, &shifted);
        }
    }

    // begin to filter out infinite points
    let mut bounded: Vec<Point> = Vec::new();

    'outer: for point in shifted {
        let repr = format!("({},{})", point.x, point.y);
        // points at the edge of the graph are by definition infinite
        if point.x == 0 || point.y == 0 || point.x == upper_bound.x || point.y == upper_bound.y {
            continue;
        }
        let min_x = &board[0][point.y as usize];
        let max_x = &board.last().unwrap()[point.y as usize];

        let min_y = &board[point.x as usize][0];
        let max_y = &board[point.x as usize].last().unwrap();

        // if we look at the edges of the board and find that our Point has been marked there, then
        // we have no interruptions bewtween the Point and the edge, so the Point is infinite.
        let bounds = vec![min_x, max_x, min_y, max_y];
        for bound in bounds {
            if repr == *bound {
                continue 'outer;
            }
        }
        bounded.push(point);
    }

    let mut biggest_size = 0;

    for point in bounded {
        let repr = format!("({},{})", point.x, point.y);
        let mut counter = 1;
        for x in &board {
            for y in x {
                if *y == repr {
                    counter += 1;
                }
            }
        }

        println!("{} {}", repr, counter);

        if counter > biggest_size {
            biggest_size = counter;
        }
    }
    println!("{:?}", board);
    println!("{}", biggest_size);
}

fn main() {
    part_one();
}
