use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
enum Cell {
    Open,
    Tree,
}

#[derive(Debug)]
struct Board {
    field: Vec<Vec<Cell>>,
}

fn read_input() -> Board {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    let mut board = Board { field: Vec::new() };
    board.field = f
        .lines()
        .into_iter()
        .map(|line| {
            let mut row = Vec::new();
            for c in line.unwrap().chars() {
                match c {
                    '#' => row.push(Cell::Tree),
                    '.' | _ => row.push(Cell::Open),
                }
            }
            row
        })
        .collect();
    board
}

#[derive(Copy, Clone)]
struct Movement {
    right: usize,
    down: usize,
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

fn part_one(board: &Board, movement: Movement) -> usize {
    let width = board.field[0].len();
    let height = board.field.len();
    let mut position = Position { x: 0, y: 0 };
    let mut result = 0;
    loop {
        let mut next_step = Position {
            x: position.x + movement.right,
            y: position.y + movement.down,
        };

        // stop looping after we've traversed the entire field
        if next_step.y > height {
            break;
        }

        // if we're beyond the width of the board remove the width amount to "loop" back around.
        if next_step.x >= width {
            next_step.x = next_step.x - width;
        }

        match &board.field[position.y][position.x] {
            Cell::Tree => result += 1,
            _ => (),
        }

        position = next_step;
    }
    result
}

fn main() {
    let board = read_input();

    // part 1
    let movement = Movement { right: 3, down: 1 };
    let part_one_result = part_one(&board, movement);
    println!("part one: {}", part_one_result);

    // part 2
    let movements = vec![
        Movement { right: 1, down: 1 },
        Movement { right: 3, down: 1 },
        Movement { right: 5, down: 1 },
        Movement { right: 7, down: 1 },
        Movement { right: 1, down: 2 },
    ];
    let part_two_result: usize = movements.iter().map(|m| part_one(&board, *m)).product();
    println!("part two: {}", part_two_result);
}
