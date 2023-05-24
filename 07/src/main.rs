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

/*
impl Operation {
    pub fn new<T>(constructor: T, argument: U) -> Self
    where
        T: Fn
}
*/

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

    let capture_to_arg = |c: &Captures, i: usize| {
        Argument::from_str(c.get(i).unwrap().as_str())
    };
    let capture_to_wire = |c: &Captures, i: usize| {
        if let Argument::WireVariant(w) = capture_to_arg(c, i) {
            Some(w)
        }
        else {
            None
        }
    };
    for line in input.lines() {
        let mut inst = Option::<Instruction>::None;
        if let Some(c) = re_assignment.captures(line) {
            inst = Some(Instruction{
                op: Operation::Assignment,
                arg1: capture_to_arg(&c, 1),
                arg2: None,
                out: capture_to_wire(&c, 2).unwrap(),
            })
        } else if let Some(c) = re_not.captures(line) {
            inst = Some(Instruction{
                op: Operation::Not,
                arg1: capture_to_arg(&c, 2),
                arg2: None,
                out: capture_to_wire(&c, 3).unwrap(),
            })
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
    let instructions = get_instructions(input).unwrap();
    HashMap::new()
}

fn part_two(input: &str) -> HashMap<&str, u16> {
    let instructions = get_instructions(input).unwrap();
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
