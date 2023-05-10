use common;
use md5;

fn part_one(input: &str) -> u32 {
    let mut result = 0;
    let mut condition = true;

    while condition {
        result += 1;
        let md5_hash = md5::compute(String::from(input)+ &result.to_string());
        condition = !format!("{:x}", md5_hash).starts_with("00000");
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
        assert_eq!(part_one("abcdef"), 609043);
        assert_eq!(part_one("pqrstuv"), 1048970);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("abcdef"), 0);
    }
}
