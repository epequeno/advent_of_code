use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn read_input(file_name: &str) -> Vec<String> {
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);
    f.lines().into_iter().map(|l| l.unwrap()).collect()
}

#[derive(Debug, Clone)]
struct SeatParseError;

#[derive(Debug)]
struct Seat {
    row: usize,
    column: usize,
}

impl Seat {
    fn seat_id(&self) -> usize {
        (self.row * 8) + self.column
    }
}

impl FromStr for Seat {
    type Err = SeatParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows: Vec<u8> = (0..=127).collect();
        let mut columns: Vec<u8> = (0..=8).collect();
        for c in s.chars() {
            match c {
                'F' => rows = rows[..rows.len() / 2].to_vec(),
                'B' => rows = rows[rows.len() / 2..].to_vec(),
                'L' => columns = columns[..columns.len() / 2].to_vec(),
                'R' => columns = columns[columns.len() / 2..].to_vec(),
                _ => (),
            }
        }
        Ok(Seat {
            row: rows[0] as usize,
            column: columns[0] as usize,
        })
    }
}

fn part_one(input: &Vec<String>) -> usize {
    let mut res = 0;
    for l in input {
        match l.parse::<Seat>() {
            Ok(seat) => {
                if seat.seat_id() > res {
                    res = seat.seat_id();
                }
            }
            Err(_) => (),
        }
    }
    res
}

fn part_two(input: &Vec<String>) -> usize {
    let mut seat_ids = Vec::new();
    for l in input {
        let seat = l.parse::<Seat>().unwrap();
        seat_ids.push(seat.seat_id());
    }
    
    let mut my_seat = 0;
    for id in *seat_ids.iter().min().unwrap()..*seat_ids.iter().max().unwrap() {
        if !seat_ids.contains(&id) {
            my_seat = id;
        }
    }
    my_seat
}
fn main() {
    let input = read_input("input.txt");
    let result = part_one(&input);
    println!("part one: {}", result);

    let result = part_two(&input);
    println!("part two: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_case_1() {
        let test_input = vec!["BFFFBBFRRR".to_string()];
        assert_eq!(part_one(&test_input), 567);
    }

    #[test]
    fn test_part_one_case_2() {
        let test_input = vec!["FFFBBBFRRR".to_string()];
        assert_eq!(part_one(&test_input), 119);
    }

    #[test]
    fn test_part_one_case_3() {
        let test_input = vec!["BBFFBBFRLL".to_string()];
        assert_eq!(part_one(&test_input), 820);
    }
}
