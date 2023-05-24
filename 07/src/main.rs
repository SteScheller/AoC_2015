use common;
use regex::{Captures, Regex};
use std::{collections::HashMap, fmt};

struct Wire(String);

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
            Argument::WireVariant(Wire(s.to_string()))
        }
    }
}

#[derive(Debug)]
enum Operation {
    Assignment,
    Not,
    LeftShift(u8),
    RightShift(u8),
    And,
    Or,
}

impl Operation {
    pub fn compute(&self, arg1: &Argument, arg2: &Option<Argument>) -> Option<u16> {
        match &self {
            Operation::Assignment => match arg1 {
                Argument::ConstantVariant(c) => Some(c.0),
                Argument::WireVariant(_) => None,
            },
            Operation::Not => match arg1 {
                Argument::ConstantVariant(c) => Some(!c.0),
                Argument::WireVariant(_) => None,
            },
            Operation::LeftShift(i) => match arg1 {
                Argument::ConstantVariant(c) => Some(c.0 << i),
                Argument::WireVariant(_) => None,
            },
            Operation::RightShift(i) => match arg1 {
                Argument::ConstantVariant(c) => Some(c.0 >> i),
                Argument::WireVariant(_) => None,
            },
            Operation::And => {
                if let (Argument::ConstantVariant(c1), Some(Argument::ConstantVariant(c2))) =
                    (arg1, arg2)
                {
                    Some(c1.0 & c2.0)
                } else {
                    None
                }
            }
            Operation::Or => {
                if let (Argument::ConstantVariant(c1), Some(Argument::ConstantVariant(c2))) =
                    (arg1, arg2)
                {
                    Some(c1.0 | c2.0)
                } else {
                    None
                }
            }
        }
    }
}

struct Instruction {
    op: Operation,
    arg1: Argument,
    arg2: Option<Argument>,
    out: Wire,
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match &self.op {
            Operation::Assignment => "".to_string(),
            Operation::And => " && ".to_string(),
            Operation::Or => " || ".to_string(),
            Operation::LeftShift(i) => format!(" << {i}"),
            Operation::RightShift(i) => format!(" >> {i}"),
            Operation::Not => "_not".to_string(),
        };
        let arg1 = match &self.arg1 {
            Argument::ConstantVariant(c) => c.0.to_string(),
            Argument::WireVariant(w) => w.0.clone(),
        };
        let arg2 = match &self.arg2 {
            None => "".to_string(),
            Some(arg) => match arg {
                Argument::ConstantVariant(c) => c.0.to_string(),
                Argument::WireVariant(w) => w.0.clone(),
            },
        };
        write!(f, "{} = {arg1}{op}{arg2};", self.out.0)
    }
}

fn get_instructions(input: &str) -> Option<Vec<Instruction>> {
    let mut instructions = Vec::new();
    let re_assignment = Regex::new(r"^(\w+) -> (\w+)").unwrap();
    let re_not = Regex::new(r"^(NOT (\w+)) -> (\w+)").unwrap();
    let re_shift = Regex::new(r"(\w+) (LSHIFT|RSHIFT) (\d+) -> (\w+)").unwrap();
    let re_other = Regex::new(r"(\w+) (AND|OR) (\w+) -> (\w+)").unwrap();

    let capture_to_arg = |c: &Captures, i: usize| Argument::from_str(c.get(i).unwrap().as_str());
    let capture_to_wire = |c: &Captures, i: usize| {
        if let Argument::WireVariant(w) = capture_to_arg(c, i) {
            Some(w)
        } else {
            None
        }
    };
    for line in input.lines() {
        let mut inst = Option::<Instruction>::None;
        if let Some(c) = re_assignment.captures(line) {
            inst = Some(Instruction {
                op: Operation::Assignment,
                arg1: capture_to_arg(&c, 1),
                arg2: None,
                out: capture_to_wire(&c, 2).unwrap(),
            })
        } else if let Some(c) = re_not.captures(line) {
            inst = Some(Instruction {
                op: Operation::Not,
                arg1: capture_to_arg(&c, 2),
                arg2: None,
                out: capture_to_wire(&c, 3).unwrap(),
            })
        } else if let Some(c) = re_shift.captures(line) {
            let op = match c.get(2).unwrap().as_str() {
                "LSHIFT" => Operation::LeftShift(c.get(3).unwrap().as_str().parse::<u8>().unwrap()),
                "RSHIFT" => {
                    Operation::RightShift(c.get(3).unwrap().as_str().parse::<u8>().unwrap())
                }
                _ => panic!("failed to parse shift instruction"),
            };
            inst = Some(Instruction {
                op: op,
                arg1: capture_to_arg(&c, 1),
                arg2: None,
                out: capture_to_wire(&c, 4).unwrap(),
            })
        } else if let Some(c) = re_other.captures(line) {
            let op = match c.get(2).unwrap().as_str() {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                _ => panic!("failed to parse instruction"),
            };
            inst = Some(Instruction {
                op: op,
                arg1: capture_to_arg(&c, 1),
                arg2: Some(capture_to_arg(&c, 3)),
                out: capture_to_wire(&c, 4).unwrap(),
            })
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

fn part_one(input: &str) -> HashMap<String, u16> {
    let instructions = get_instructions(input).unwrap();
    let mut results: HashMap<String, u16> = HashMap::new();
    while results.len() < instructions.len() {
        for inst in &instructions {
            if !results.contains_key(&inst.out.0) {
                let arg1 = results.contains_key(k)
                let (mut arg1, mut arg2) = (0, 0);
                match inst.arg1 {
                    Argument::ConstantVariant(c) => {
                        arg1 = c.0;
                    }
                    Argument::WireVariant(w) => {
                        if results.contains_key(&w.0) {
                            arg1 = results.get(&w.0).unwrap().clone();
                        } else {
                            continue;
                        }
                    }
                };
                match inst.arg2 {
                    None => {}
                    Some(arg) => match arg {
                        Argument::ConstantVariant(c) => {
                            arg2 = c.0;
                        }
                        Argument::WireVariant(w) => {
                            if results.contains_key(&w.0) {
                                arg2 = results.get(&w.0).unwrap().clone();
                            } else {
                                continue;
                            }
                        }
                    },
                };
                if let Some(value) = inst.op.compute(arg1, arg2) {
                    results.insert(inst.out.0.clone(), value);
                }
            }
        }
    }
    results
}

fn part_two(_input: &str) -> HashMap<String, u16> {
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
            ("d".to_string(), 72),
            ("e".to_string(), 507),
            ("f".to_string(), 492),
            ("g".to_string(), 114),
            ("h".to_string(), 65412),
            ("i".to_string(), 65079),
            ("x".to_string(), 123),
            ("y".to_string(), 456),
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
