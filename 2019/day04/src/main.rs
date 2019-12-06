// https://adventofcode.com/2019/day/4

fn has_two_adjacent(n: usize) -> bool {
    let chars: Vec<char> = n.to_string().chars().collect();
    for i in 0..chars.len() {
        if i + 1 >= chars.len() {
            break;
        }
        if chars[i] == chars[i + 1] {
            return true;
        }
    }
    false
}

fn does_not_decrease(n: usize) -> bool {
    let chars: Vec<char> = n.to_string().chars().collect();
    for i in 0..chars.len() {
        if i + 1 >= chars.len() {
            break;
        }
        let x: u32 = chars[i].to_digit(10).unwrap();
        let y: u32 = chars[i + 1].to_digit(10).unwrap();
        if x > y {
            return false;
        }
    }
    true
}

fn has_small_match_group(n: usize) -> bool {
    let chars: Vec<char> = n.to_string().chars().collect();

    // first find which chars are potential matches
    let mut candidates: Vec<char> = Vec::new();
    for i in 0..chars.len() {
        if i + 1 >= chars.len() {
            break;
        }
        let x = chars[i];
        let y = chars[i + 1];
        if x == y {
            if !candidates.contains(&x) {
                candidates.push(x);
            }
        }
    }

    let mut consecutive = 0;
    for candidate in candidates {
        consecutive = 0;
        let mut start = false;
        for i in 0..chars.len() {
            if candidate == chars[i] && !start {
                // we found a match, start counting consecutives
                start = true;
                consecutive += 1;
            } else if candidate == chars[i] && start {
                // we previously started counting, continue doing so here
                consecutive += 1;
            } else if candidate != chars[i] && start {
                // we found a mismatch and had already started counting
                if consecutive == 2 {
                    // if we found exactly two consecutive and we've now found something different
                    // we can simply stop here, whatever happens the rest of the way won't change
                    // the result.
                    return true;
                }
                // we found a difference and were already counting, we won't find any more
                // consecutive at this point, break.
                break;
            }
        }

        // we found all the matches we could for this candidate, if it's more than 2 then it's not
        // a candidate we care about
        if consecutive > 2 {
            continue;
        }
    }

    // if we made it this far and haven't hit any previous conditions simply test if the last thing
    // we checked had exactly two matches. This will probably match most often when the pair is the
    // last two chars.
    consecutive == 2
}

fn main() {
    let input = 193651..649729;
    let res = input
        .clone()
        .filter(|i| has_two_adjacent(*i) && does_not_decrease(*i))
        .count();
    println!("part one = {}", res);

    let res = input
        .clone()
        .filter(|i| has_two_adjacent(*i) && does_not_decrease(*i) && has_small_match_group(*i))
        .count();
    println!("part two = {}", res);
}
