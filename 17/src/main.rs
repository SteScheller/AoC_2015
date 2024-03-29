use std::cmp::{min, Ordering};

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

fn find_min_num_containers(remaining: i32, summands: &[u32], acc: u32) -> Option<u32> {
    match remaining.cmp(&0) {
        Ordering::Less => None,
        Ordering::Equal => Some(acc),
        Ordering::Greater => {
            if summands.is_empty() {
                None
            } else {
                let with = find_min_num_containers(
                    remaining - summands[0] as i32,
                    &summands[1..],
                    acc + 1,
                );
                let without = find_min_num_containers(remaining, &summands[1..], acc);
                match (with, without) {
                    (Some(a), Some(b)) => Some(min(a, b)),
                    (Some(a), None) => Some(a),
                    (None, Some(b)) => Some(b),
                    (None, None) => None,
                }
            }
        }
    }
}

fn find_min_combinations(remaining: i32, summands: &[u32], acc: u32, min_num: u32) -> u32 {
    match remaining.cmp(&0) {
        Ordering::Less => 0,
        Ordering::Equal => {
            if acc <= min_num {
                1
            } else {
                0
            }
        }
        Ordering::Greater => {
            if summands.is_empty() {
                0
            } else {
                let with = find_min_combinations(
                    remaining - summands[0] as i32,
                    &summands[1..],
                    acc + 1,
                    min_num,
                );
                let without = find_min_combinations(remaining, &summands[1..], acc, min_num);
                with + without
            }
        }
    }
}

fn part_one(input: &str, sum_eggnog: u32) -> u32 {
    let containers = get_containers(input);
    find_combinations(sum_eggnog as i32, &containers)
}

fn part_two(input: &str, sum_eggnog: u32) -> u32 {
    let containers = get_containers(input);
    let min = find_min_num_containers(sum_eggnog as i32, &containers, 0).unwrap();
    find_min_combinations(sum_eggnog as i32, &containers, 0, min)
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
        assert_eq!(find_combinations(25, &[20, 15, 10, 5, 5]), 4);
    }

    #[test]
    fn test_find_min_num_containers() {
        assert_eq!(find_min_num_containers(25, &[20, 15, 10, 5, 5], 0), Some(2));
    }
}
