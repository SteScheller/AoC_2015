use std::fs;

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}

fn part_one(input: &str) -> i32 {
    let mut result = 0;
    for c in input.chars() {
        match c {
            '(' => result += 1,
            ')' => result -= 1,
            _ => (),
        }
    }

    result
}

fn part_two(input: &str) -> usize {
    let mut floor = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            return i + 1;
        }
    }
    0
}

fn main() {
    let input = read_input("input.txt");
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert!((part_one("(())") == part_one("()()")) && part_one("(())") == 0);
        assert!(
            (part_one("(((") == part_one("(()(()("))
                && (part_one("(((") == part_one("))((((("))
                && part_one("(((") == 3
        );
        assert!((part_one("())") == part_one("))(")) && part_one("())") == -1);
        assert!((part_one(")))") == part_one(")())())")) && part_one(")))") == -3);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(")"), 1);
        assert_eq!(part_two("()())"), 5);
    }
}
