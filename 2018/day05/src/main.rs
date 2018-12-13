use std::fs::File;
use std::io::prelude::*;

fn will_react(a: char, b: char) -> bool {
    let a_lower = a.to_lowercase().to_string();
    let b_lower = b.to_lowercase().to_string();
    (a_lower == b_lower)
        && ((a.is_uppercase() && b.is_lowercase()) || (a.is_lowercase() && b.is_uppercase()))
}

fn clean(mut chars: Vec<char>) -> String {
    let mut i = 0;
    let mut j = 1;

    let mut made_change = false;
    loop {
        if j == chars.len() {
            // we've hit the end of the vec, start again. If we've made it this far and haven't
            // made a change since the last time we reset the flag to false, then no more changes
            // are needed.
            if !made_change {
                break;
            } else {
                made_change = false;
                i = 0;
                j = 1;
            }
        }

        let a = chars[i];
        let b = chars[j];

        if will_react(a, b) {
            chars.remove(i);
            chars.remove(i);
            made_change = true;
            continue;
        } else {
            i += 1;
            j += 1;
        }
    }
    chars.into_iter().collect()
}

fn part_one() {
    let mut f = File::open("input.txt").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();
    let chars: Vec<char> = data.chars().collect();
    let ans = clean(chars);
    println!("{} - {}", ans.len(), ans);
}

fn part_two() {
    let mut f = File::open("input2.txt").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();
    let chars: Vec<char> = data.chars().collect();
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    for letter in alphabet {
        let upper = letter.to_ascii_uppercase();
        let test: String = chars.clone().into_iter().collect();
        let test = test.replace(letter, "").replace(upper, "");
        let ans = clean(test.chars().collect());
        println!("{} - {}", letter, ans.len());
    }
}

fn main() {
    part_one();
    part_two();
}
