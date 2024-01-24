use regex::{Captures, Regex};

struct LightGrid {
    width: usize,
    height: usize,
    arr: Vec<Vec<u32>>,
}

impl LightGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let arr = vec![vec![0; height]; width];
        Self { width, height, arr }
    }

    pub fn get_num_switched_on(&self) -> usize {
        let mut result = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                if self.arr[x][y] > 0 {
                    result += 1;
                }
            }
        }
        result
    }

    pub fn get_brightness(&self) -> u32 {
        let mut result = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                result += self.arr[x][y];
            }
        }
        result
    }

    pub fn turn_on_off(&mut self, instr: &Instruction) {
        match *instr {
            Instruction::TurnOn((x1, y1), (x2, y2)) => {
                for x in x1..x2 + 1 {
                    for y in y1..y2 + 1 {
                        self.arr[x][y] = 1;
                    }
                }
            }
            Instruction::TurnOff((x1, y1), (x2, y2)) => {
                for x in x1..x2 + 1 {
                    for y in y1..y2 + 1 {
                        self.arr[x][y] = 0;
                    }
                }
            }
            Instruction::Toggle((x1, y1), (x2, y2)) => {
                for x in x1..x2 + 1 {
                    for y in y1..y2 + 1 {
                        self.arr[x][y] = !self.arr[x][y];
                    }
                }
            }
        }
    }

    pub fn dim_brightness(&mut self, instr: &Instruction) {
        match *instr {
            Instruction::TurnOn((x1, y1), (x2, y2)) => {
                for x in x1..x2 + 1 {
                    for y in y1..y2 + 1 {
                        self.arr[x][y] += 1;
                    }
                }
            }
            Instruction::TurnOff((x1, y1), (x2, y2)) => {
                for x in x1..x2 + 1 {
                    for y in y1..y2 + 1 {
                        if self.arr[x][y] > 0 {
                            self.arr[x][y] -= 1;
                        }
                    }
                }
            }
            Instruction::Toggle((x1, y1), (x2, y2)) => {
                for x in x1..x2 + 1 {
                    for y in y1..y2 + 1 {
                        self.arr[x][y] += 2;
                    }
                }
            }
        }
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
}

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

fn part_one(input: &str) -> usize {
    let instructions = get_instructions(input).unwrap();
    let mut lights = LightGrid::new(1000, 1000);
    for inst in instructions {
        lights.turn_on_off(&inst);
    }
    lights.get_num_switched_on()
}

fn part_two(input: &str) -> u32 {
    let instructions = get_instructions(input).unwrap();
    let mut lights = LightGrid::new(1000, 1000);
    for inst in instructions {
        lights.dim_brightness(&inst);
    }
    lights.get_brightness()
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
        assert_eq!(part_two("turn on 0,0 through 0,0"), 1);
        assert_eq!(part_two("toggle 0,0 through 999,999"), 2_000_000);
    }
}
