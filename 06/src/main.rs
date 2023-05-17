use common;
use regex::{Captures, Regex};

struct LightGrid;

impl LightGrid {
    pub fn new() -> Self {
        Self
    }

    pub fn get_num_switched_on(&self) -> usize {
        0
    }
}

enum Instruction {
    TurnOn((usize, usize), (usize, usize)),
    TurnOff((usize, usize), (usize, usize)),
    Toggle((usize, usize), (usize, usize)),
}

impl Instruction {
    pub fn new<T>(constructor: T, points_tuple: ((usize, usize), (usize, usize))) -> Self
    where
        T: Fn((usize, usize), (usize, usize)) -> Self,
    {
        constructor(points_tuple.0, points_tuple.1)
    }

    pub fn apply(&self, lights: &mut LightGrid) {}
}

fn get_instructions(input: &str) -> Option<Vec<Instruction>> {
    let mut instructions = Vec::new();
    let re_turn_on = Regex::new(r"turn on ((\d+),(\d+)) through ((\d+),(\d+))").unwrap();
    let re_turn_off = Regex::new(r"turn off ((\d+),(\d+)) through ((\d+),(\d+))").unwrap();
    let re_toggle = Regex::new(r"toogle ((\d+),(\d+)) through ((\d+),(\d+))").unwrap();

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

fn part_one(input: &str) -> usize {
    let instructions = get_instructions(input).unwrap();
    let mut lights = LightGrid;
    for inst in instructions {
        inst.apply(&mut lights);
    }
    lights.get_num_switched_on()
}

fn part_two(input: &str) -> usize {
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
        assert_eq!(part_one("turn on 0,0 through 999,999"), 1_000_000);
        assert_eq!(part_one("toggle 0,0 through 999,0"), 1_000);
        assert_eq!(
            part_one("turn on 0,0 through 999,999\nturn off 499,499 through 500,500"),
            1_000_000 - 4
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(""), 1_000_000);
    }
}
