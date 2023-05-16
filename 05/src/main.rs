use common;
use regex::Regex;

fn part_one(input: &str) -> u32 {
    let mut result = 0;

    let re_three_vowels = Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
    let re_double_letter = Regex::new(r"(?P<l>\w)${l}+").unwrap();
    let re_not_contain = Regex::new(r"(ab|cd|pq|xy)").unwrap();

    for item in input.lines() {
        if let None = re_three_vowels.captures(item) {
            continue;
        };
        if let None = re_double_letter.captures(item) {
            continue;
        };
        if let Some(_) = re_not_contain.captures(item) {
            continue;
        };
        result += 1;
    }
    result
}

fn part_two(input: &str) -> u32 {
    0
}

fn main() {
    let input = common::read_input("input.txt");
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("ugknbfddgicrmopn"), 1);
        assert_eq!(part_one("aaa"), 1);
        assert_eq!(part_one("jchzalrnumimnmhp"), 0);
        assert_eq!(part_one("haegwjzuvuyypxyu"), 0);
        assert_eq!(part_one("dvszwmarrgswjxmb"), 0);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(""), 0);
    }
}
