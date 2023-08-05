use common;

fn part_one(_input: &str) -> u32 {
    0
}

fn part_two(_input: &str) -> u32 {
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
        assert_eq!(part_one("[1,2,3]"), 6);
        assert_eq!(part_one("{\"a\": 2, \"b\":4}"), 6);
        assert_eq!(part_one("[[[3]]]"), 3);
        assert_eq!(part_one("{\"a\":{\"b\":4},\"c\":-1}"), 3);
        assert_eq!(part_one("{\"a\":[-1,1]}"), 0);
        assert_eq!(part_one("[-1,{\"a\":1}]"), 0);
        assert_eq!(part_one("[]"), 0);
        assert_eq!(part_one("{}"), 0);
    }

    #[test]
    fn test_part_two() {
        assert!(false);
    }
}
