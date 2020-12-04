use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

#[derive(Debug, Clone)]
struct PassportParseError;

impl std::fmt::Display for PassportParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "bad passport")
    }
}

impl FromStr for Passport {
    type Err = PassportParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(' ').collect();
        // set default values
        let mut byr = String::new();
        let mut iyr = String::new();
        let mut eyr = String::new();
        let mut hgt = String::new();
        let mut hcl = String::new();
        let mut ecl = String::new();
        let mut pid = String::new();
        let mut cid = String::new();

        // try to pull actual values from input
        for prop in split {
            let split: Vec<&str> = prop.split(':').collect();
            let key = split[0];
            let value = split[1];
            match key {
                "byr" => {
                    byr = {
                        let year: usize = value.parse().unwrap();
                        if 1920 <= year && year <= 2002 {
                            value.to_string()
                        } else {
                            return Err(PassportParseError);
                        }
                    }
                }
                "iyr" => {
                    iyr = {
                        let year: usize = value.parse().unwrap();
                        if 2010 <= year && year <= 2020 {
                            value.to_string()
                        } else {
                            return Err(PassportParseError);
                        }
                    }
                }
                "eyr" => {
                    eyr = {
                        let year: usize = value.parse().unwrap();
                        if 2020 <= year && year <= 2030 {
                            value.to_string()
                        } else {
                            return Err(PassportParseError);
                        }
                    }
                }
                "hgt" => {
                    hgt = {
                        match value[..value.len() - 2].parse::<usize>() {
                            Ok(height_value) => match &value[value.len() - 2..] {
                                "cm" => {
                                    if 150 <= height_value && height_value <= 193 {
                                        value.to_string()
                                    } else {
                                        return Err(PassportParseError);
                                    }
                                }
                                "in" => {
                                    if 59 <= height_value && height_value <= 76 {
                                        value.to_string()
                                    } else {
                                        return Err(PassportParseError);
                                    }
                                }
                                _ => return Err(PassportParseError),
                            },
                            Err(_) => return Err(PassportParseError),
                        }
                    }
                }
                "hcl" => {
                    hcl = {
                        if !value.starts_with('#') || value.len() != 7 {
                            return Err(PassportParseError);
                        } else {
                            if value[1..].chars().all(|c| c.is_digit(16)) {
                                value.to_string()
                            } else {
                                return Err(PassportParseError);
                            }
                        }
                    }
                }
                "ecl" => {
                    ecl = {
                        match value {
                            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {
                                value.to_string()
                            }
                            _ => return Err(PassportParseError),
                        }
                    }
                }
                "pid" => {
                    pid = {
                        if value.len() != 9 || !value.chars().all(|c| c.is_digit(10)) {
                            return Err(PassportParseError);
                        } else {
                            value.to_string()
                        }
                    }
                }
                "cid" => cid = value.to_string(),
                _ => (),
            }
        }

        // check for required values
        let required_values = vec![&byr, &iyr, &eyr, &hgt, &hcl, &ecl, &pid];
        if required_values.iter().any(|p| p.is_empty()) {
            Err(PassportParseError)
        } else {
            Ok(Passport {
                byr,
                iyr,
                eyr,
                hgt,
                hcl,
                ecl,
                pid,
                cid,
            })
        }
    }
}

fn read_input(file_name: &str) -> Vec<String> {
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);
    let lines: Vec<String> = f.lines().into_iter().map(|l| l.unwrap()).collect();
    let mut res = Vec::new();
    let mut passport: Vec<String> = Vec::new();
    for line in lines {
        if line.is_empty() {
            let combined = passport.join(" ");
            res.push(combined);
            passport = Vec::new();
            continue;
        }
        passport.push(String::from(line.trim_end()));
    }
    // since the input doesn't end on a newline we'll add the last thing we saw
    let combined = passport.join(" ");
    res.push(combined);
    res
}

fn count_valid_passports(input: &Vec<String>) -> usize {
    let mut res = 0;
    for p in input {
        match p.parse::<Passport>() {
            Ok(_) => res += 1,
            Err(_) => (),
        }
    }
    res
}

fn main() {
    let input = read_input("input.txt");
    let res = count_valid_passports(&input);
    println!("part two: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_valid_passports() {
        let test_input = read_input("test_input_part_one.txt");
        assert_eq!(count_valid_passports(&test_input), 2);
    }

    #[test]
    fn test_part_two_invalid_passports() {
        let test_input = read_input("test_input_part_two_invalid.txt");
        assert_eq!(count_valid_passports(&test_input), 0);
    }

    #[test]
    fn test_part_two_valid_passports() {
        let test_input = read_input("test_input_part_two_valid.txt");
        assert_eq!(count_valid_passports(&test_input), 4);
    }
}
