use std::fs::File;
use std::io::prelude::*;

fn should_react(a: char, b: char) -> bool {
    let a_lower = a.to_lowercase().to_string();
    let b_lower = b.to_lowercase().to_string();
    if a_lower != b_lower {
        false
    } else {
        (a.is_uppercase() && b.is_lowercase()) || (a.is_lowercase() && b.is_uppercase())
    }
}

fn part_one() {
    let mut f = File::open("input.txt").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();
    let mut original = data.clone();
    // let chars: Vec<char> = data.chars().collect();

    'outer: loop {
        let chars: Vec<char> = original.chars().collect();
        let mut found_match = false;
        for i in (0..(chars.len() - 1)).step_by(2) {
            let a = chars[i];
            let b = chars[i + 1];
            if should_react(a, b) {
                found_match = true;
                let to_remove = format!("{}{}", a, b);
                original = original.replacen(&to_remove, "", 1);
                continue 'outer;
            }
        }
        if !found_match {
            println!("{}", original.len());
            break;
        }
    }
}

fn main() {
    part_one();
}
