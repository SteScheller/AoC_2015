use common;

fn part_one_two(input: &str, iterations: usize) -> String {
    let mut sequence = String::from(input);

    for _ in 0..iterations {
        let input = sequence;
        let mut count = 1;
        let mut current = input.chars().nth(0).unwrap_or('\0');

        sequence = String::new();
        for c in input.chars().skip(1) {
            if c == current {
                count += 1;
            } else {
                sequence.push_str(&count.to_string());
                sequence.push(current);
                current = c;
                count = 1;
            }
        }
        sequence.push_str(&count.to_string());
        sequence.push(current);
    }

    sequence
}

fn main() {
    let input = common::read_input("input.txt");
    println!("{}", part_one_two(&input, 40).len());
    println!("{}", part_one_two(&input, 50).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_two() {
        assert_eq!(part_one_two("1", 1), "11");
        assert_eq!(part_one_two("1", 2), "21");
        assert_eq!(part_one_two("1", 3), "1211");
        assert_eq!(part_one_two("1", 4), "111221");
        assert_eq!(part_one_two("1", 5), "312211");
    }
}
