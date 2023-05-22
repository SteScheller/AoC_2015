use common;
use regex::{Captures, Regex};
use std::collections::HashMap;

enum Argument {
    Signal(String),
    Constant(u16),
}

enum Operation {
    Assignment,
    Not,
    LeftShift(u8),
    RightShift(u8),
    And,
    Or,
}

struct Instruction{
    op: Operation,
    arg1: Argument,
    arg2: Option<Argument>,
    out: Argument::Signal,
}

fn get_instructions(input: &str) -> Option<Vec<Instruction>> {
    let mut instructions = Vec::new();
    let re_assignment = Regex::new(r"^(\w+) -> (\w+)").unwrap();
    let re_not = Regex::new(r"^(NOT (\w+)) -> (\w+)").unwrap();
    let re_shift = Regex::new(r"(\w+) (LSHIFT|RSHIFT) (\d+) -> (\w+)").unwrap();
    let re_other = Regex::new(r"(\w+) (AND|OR) (\w+) -> (\w+)").unwrap();

    let captures_to_points = |c: Captures| {
        (
            (
                c.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                c.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            ),
            (
                c.get(5).unwrap().as_str().parse::<usize>().unwrap(),
                c.get(6).unwrap().as_str().parse::<usize>().unwrap(),
            ),
        )
    };

    for line in input.lines() {
        if let Some(c) = re_assignment.captures(line) {
        } else if let Some(c) = re_not.captures(line) {
        } else if let Some(c) = re_shift.captures(line) {
        } else if let Some(c) = re_other.captures(line) {
        }
    }

    match instructions.len() {
        0 => None,
        _ => Some(instructions),
    }
}

fn part_one(input: &str) -> HashMap<&str,i32> {
    let instructions = get_instructions(input).unwrap();
}

fn part_two(input: &str) -> HashMap<&str,i32> {
    let instructions = get_instructions(input).unwrap();
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
                    let expected_map=HashMap::from([
        ("d", 72),
        ("e", 507),
        ("f", 492),
        ("g", 114),
        ("h", 65412),
        ("i", 65079),
        ("x", 123),
        ("y", 456),
                    ]);
                    let computed_map = part_one(
                        "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i"
                    );

                    for (k, v) in &computed_map {
                        assert_eq!(v, expected_map.entry(k));
                    }
        };
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(""), 1);
    }
}
