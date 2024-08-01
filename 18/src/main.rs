fn part_one(input: &str) -> usize {
    0
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
        assert_eq!(part_one("2,3,0,3,10,11,12,1,1,0,1,99,2,1,1,2"), 4);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_one("2,3,0,3,10,11,12,1,1,0,1,99,2,1,1,2"), 0);
    }
}
