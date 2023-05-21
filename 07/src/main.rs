use common;
use regex::{Captures, Regex};
use std::collections::HashMap;

enum Instruction {}

fn get_instructions(input: &str) -> Option<Vec<Instruction>> {
    let mut instructions = Vec::new();
    let re_turn_on = Regex::new(r"turn on ((\d+),(\d+)) through ((\d+),(\d+))").unwrap();
    let re_turn_off = Regex::new(r"turn off ((\d+),(\d+)) through ((\d+),(\d+))").unwrap();
    let re_toggle = Regex::new(r"toggle ((\d+),(\d+)) through ((\d+),(\d+))").unwrap();

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
        if let Some(c) = re_turn_on.captures(line) {
            instructions.push(Instruction::new(Instruction::TurnOn, captures_to_points(c)));
        } else if let Some(c) = re_turn_off.captures(line) {
            instructions.push(Instruction::new(
                Instruction::TurnOff,
                captures_to_points(c),
            ));
        } else if let Some(c) = re_toggle.captures(line) {
            instructions.push(Instruction::new(Instruction::Toggle, captures_to_points(c)));
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
