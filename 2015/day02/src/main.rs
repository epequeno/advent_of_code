use std::cmp::min;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn file_to_vec(filename: &str) -> Vec<String> {
    let mut items = Vec::new();
    if let Ok(f) = File::open(filename) {
        let data = BufReader::new(f);
        for line in data.lines() {
            match line {
                Ok(l) => items.push(l),
                Err(e) => {
                    println!("error reading line: {}", e);
                    continue;
                }
            }
        }
    }
    items
}

#[derive(Debug)]
struct Present {
    length: usize,
    width: usize,
    height: usize,
    surface_area: usize,
    smallest_area: usize,
    smallest_perimeter: usize,
    volume: usize,
}

fn surface_area(length: usize, width: usize, height: usize) -> usize {
    (2 * length * width) + (2 * width * height) + (2 * height * length)
}

fn smallest_area(length: usize, width: usize, height: usize) -> usize {
    let side_a = length * width;
    let side_b = width * height;
    let side_c = height * length;
    min(min(side_a, side_b), side_c)
}

fn smallest_perimeter(length: usize, width: usize, height: usize) -> usize {
    let side_a = (2 * length) + (2 * width);
    let side_b = (2 * width) + (2 * height);
    let side_c = (2 * height) + (2 * length);
    min(min(side_a, side_b), side_c)
}

fn volume(length: usize, width: usize, height: usize) -> usize {
    length * width * height
}

impl Present {
    fn new(line: &str) -> Present {
        let parts: Vec<&str> = line.split("x").collect();
        let length = parts[0].parse().unwrap();
        let width = parts[1].parse().unwrap();
        let height = parts[2].parse().unwrap();
        Present {
            length,
            width,
            height,
            surface_area: surface_area(length, width, height),
            smallest_area: smallest_area(length, width, height),
            smallest_perimeter: smallest_perimeter(length, width, height),
            volume: volume(length, width, height),
        }
    }
}

fn main() {
    let presents = file_to_vec("input.txt");
    let presents: Vec<Present> = presents.into_iter().map(|p| Present::new(&p)).collect();
    let mut part_one = 0;
    for present in &presents {
        part_one += present.surface_area + present.smallest_area;
    }
    println!("{}", part_one);

    let mut part_two = 0;
    for present in &presents {
        part_two += present.smallest_perimeter + present.volume;
    }
    println!("{}", part_two);
}
