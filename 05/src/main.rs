use fancy_regex::Regex;

fn part_one(input: &str) -> u32 {
    let mut result = 0;

    let re_three_vowels = Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
    let re_double_letter = Regex::new(r"(\w)\1+").unwrap();
    let re_naughty_sequences = Regex::new(r"(ab|cd|pq|xy)").unwrap();

    for item in input.lines() {
        if !re_three_vowels.is_match(item).unwrap() {
            continue;
        }
        if !re_double_letter.is_match(item).unwrap() {
            continue;
        }
        if re_naughty_sequences.is_match(item).unwrap() {
            continue;
        }
        result += 1;
    }
    result
}

fn part_two(input: &str) -> u32 {
    let mut result = 0;

    let re_double_pair = Regex::new(r"(\w\w).*\1").unwrap();
    let re_repeated_letter = Regex::new(r"(\w).\1").unwrap();

    for item in input.lines() {
        if !re_double_pair.is_match(item).unwrap() {
            continue;
        }
        if !re_repeated_letter.is_match(item).unwrap() {
            continue;
        }
        result += 1;
    }
    result
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
        assert_eq!(part_two("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(part_two("xxyxx"), 1);
        assert_eq!(part_two("uurcxstgmygtbstg"), 0);
        assert_eq!(part_two("ieodomkazucvgmuy"), 0);
    }
}
