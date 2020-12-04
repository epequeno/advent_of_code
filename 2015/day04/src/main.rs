// https://gist.github.com/gkbrk/2e4835e3a17b3fb6e1e7
use crypto::digest::Digest;
use crypto::md5::Md5;

fn find_num(input: &str, prefix: &str) -> usize {
    let mut hasher = Md5::new();
    for i in 0..std::usize::MAX {
        hasher.input(input.as_bytes());
        hasher.input(i.to_string().as_bytes());
        let result = hasher.result_str();
        if result.starts_with(prefix) {
            return i;
        }
        hasher.reset();
    }
    0
}

fn main() {
    let input: &str = "ckczppom";
    println!("part one: {}", find_num(&input, "00000"));
    println!("part two: {}", find_num(&input, "000000"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_case_1() {
        assert_eq!(find_num(&"abcdef", "00000"), 609043);
    }

    #[test]
    fn test_part_one_case_2() {
        assert_eq!(find_num(&"pqrstuv", "00000"), 1048970);
    }
}
