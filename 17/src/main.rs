use std::cmp::Ordering;

fn get_containers(input: &str) -> Vec<u32> {
    let mut containers = Vec::new();

    for line in input.lines() {
        containers.push(line.parse().unwrap());
    }

    containers
}

fn find_combinations(remaining: i32, summands: &[u32]) -> u32 {
    match remaining.cmp(&0) {
        Ordering::Less => 0,
        Ordering::Equal => 1,
        Ordering::Greater => {
            if summands.is_empty() {
                0
            } else {
                let with = find_combinations(remaining - summands[0] as i32, &summands[1..]);
                let without = find_combinations(remaining, &summands[1..]);
                with + without
            }
        }
    }
}

fn part_one(input: &str, sum_eggnog: u32) -> u32 {
    let containers = get_containers(input);
    find_combinations(sum_eggnog as i32, &containers)
}

fn part_two(_input: &str, _sum_eggnog: u32) -> u32 {
    0
}

fn main() {
    let input = common::read_input("input.txt");
    println!("{}", part_one(&input, 150));
    println!("{}", part_two(&input, 150));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_combinations() {
        assert_eq!(find_combinations(25, &[20, 15, 10, 5, 5]), 4)
    }
}
