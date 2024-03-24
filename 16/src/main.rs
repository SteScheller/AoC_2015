use regex::Regex;
use std::collections::HashMap;

struct Sue {
    number: u32,
    properties: HashMap<String, u32>,
}

impl Sue {
    pub fn new(number: u32, properties: HashMap<String, u32>) -> Self {
        Sue { number, properties }
    }
}

fn get_sues(input: &str) -> Vec<Sue> {
    let sues = Vec::new();
    let re_number = Regex::new(r"Sue (\d+)").unwrap();
    let re_property = Regex::new(r"(\w+): (\d+)").unwrap();

    for line in input.lines() {}

    sues
}

fn part_one(input: &str) -> u32 {
    0
}

fn part_two(_input: &str) -> u32 {
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
}
