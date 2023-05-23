use common;
use regex::{Captures, Regex};
use std::collections::HashMap;

struct Wire {
    name: String,
    signal: u16,
}
struct Constant(u16);

enum Argument {
    WireVariant(Wire),
    ConstantVariant(Constant),
}

impl Argument {
    pub fn from_str(s: &str) -> Self {
        let value = s.parse::<u16>();
        if let Ok(value) = value {
            Argument::ConstantVariant(Constant(value))
        } else {
            Argument::WireVariant(Wire {
                name: s.to_string(),
                signal: 0,
            })
        }
    }
}

enum Operation {
    Assignment,
    Not,
    LeftShift(u8),
    RightShift(u8),
    And,
    Or,
}

struct Instruction {
    op: Operation,
    arg1: Argument,
    arg2: Option<Argument>,
    out: Wire,
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
        let mut inst = Option::<Instruction>::None;
        if let Some(c) = re_assignment.captures(line) {
        } else if let Some(c) = re_not.captures(line) {
        } else if let Some(c) = re_shift.captures(line) {
        } else if let Some(c) = re_other.captures(line) {
        }

        if let Some(inst) = inst {
            instructions.push(inst);
        }
    }

    match instructions.len() {
        0 => None,
        _ => Some(instructions),
    }
}

fn part_one(input: &str) -> HashMap<&str, u16> {
    //let instructions = get_instructions(input).unwrap();
    HashMap::new()
}

fn part_two(input: &str) -> HashMap<&str, u16> {
    //let instructions = get_instructions(input).unwrap();
    HashMap::new()
}

fn main() {
    let input = common::read_input("input.txt");
    println!("{}", part_one(&input).get("a").unwrap_or(&0));
    println!("{}", part_two(&input).get("a").unwrap_or(&0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let expected_map = HashMap::from([
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
NOT y -> i",
        );

        for (k, v) in &expected_map {
            assert_eq!(v, computed_map.get(k).unwrap_or(&(v + 1)));
        }
    }

    #[test]
    fn test_part_two() {
        assert!(false);
    }
}
