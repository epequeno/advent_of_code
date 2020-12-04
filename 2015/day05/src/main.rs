use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_input(file_name: &str) -> Vec<String> {
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);
    f.lines().into_iter().map(|l| l.unwrap()).collect()
}

#[derive(PartialEq)]
enum SantaString {
    Naughty,
    Nice,
}

fn determine_naughtiness(input: &str) -> SantaString {
    if input == "" {
        SantaString::Naughty
    } else {
        SantaString::Nice
    }
}

fn part_one(input: &Vec<String>) -> usize {
    input
        .iter()
        .filter(|s| determine_naughtiness(s) == SantaString::Nice)
        .count()
}

fn main() {
    let input = read_input("input.txt");
    println!("part one: {}", part_one(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_group_of_strings() {
        // part_one should return the number of 'Nice' strings
        let test_strings = vec![
            "ugknbfddgicrmopn".to_string(), // nice
            "aaa".to_string(),              // nice
            "jchzalrnumimnmhp".to_string(), // naughty
            "haegwjzuvuyypxyu".to_string(), // naughty
            "dvszwmarrgswjxmb".to_string(), // naughty
        ];
        assert_eq!(part_one(&test_strings), 2);
    }

    // #[test]
    // fn test_part_two_case_1() {
    //     assert_eq!(part_two(&"^v"), 3);
    // }

    // #[test]
    // fn test_part_two_case_2() {
    //     assert_eq!(part_two(&"^>v<"), 3);
    // }

    // #[test]
    // fn test_part_two_case_3() {
    //     assert_eq!(part_two(&"^v^v^v^v^v"), 11);
    // }
}
