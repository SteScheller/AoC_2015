use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
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
    let mut sues = Vec::new();
    let re_number = Regex::new(r"Sue (\d+)").unwrap();
    let re_property = Regex::new(r"(\w+): (\d+)").unwrap();

    for line in input.lines() {
        if let Some(c) = re_number.captures(line) {
            let num = c.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let mut properties = HashMap::new();
            let line_props = &line[c.get(0).unwrap().end()..];
            for c in re_property.captures_iter(line_props) {
                let prop_name = c.get(1).unwrap().as_str();
                let count = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
                properties.insert(prop_name.to_string(), count);
            }
            sues.push(Sue::new(num, properties));
        }
    }

    sues
}

fn part_one(input: &str, target_properties: &HashMap<String, u32>) -> u32 {
    let sues = get_sues(input);
    let mut number = 0;
    'sue_loop: for s in sues {
        for p in s.properties {
            let target = target_properties.get(&p.0).unwrap();
            if *target != p.1 {
                continue 'sue_loop;
            }
        }
        number = s.number;
        break 'sue_loop;
    }
    number
}

fn part_two(input: &str, target_properties: &HashMap<String, u32>) -> u32 {
    let sues = get_sues(input);
    let mut number = 0;
    'sue_loop: for s in sues {
        for p in s.properties {
            let target = target_properties.get(&p.0).unwrap();
            match p.0.as_str() {
                "cats" => {
                    if p.1 <= *target {
                        continue 'sue_loop;
                    }
                }
                "trees" => {
                    if p.1 <= *target {
                        continue 'sue_loop;
                    }
                }
                "pomeranians" => {
                    if p.1 >= *target {
                        continue 'sue_loop;
                    }
                }
                "goldfish" => {
                    if p.1 >= *target {
                        continue 'sue_loop;
                    }
                }
                _ => {
                    if p.1 != *target {
                        continue 'sue_loop;
                    }
                }
            }
        }
        number = s.number;
        break 'sue_loop;
    }
    number
}

fn main() {
    let input = common::read_input("input.txt");
    let target_properties = HashMap::from([
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1),
    ]);

    println!("{}", part_one(&input, &target_properties));
    println!("{}", part_two(&input, &target_properties));
}

#[cfg(test)]
mod tests {
    use super::*;
    use facilitest::*;

    test_p! {
        get_sues,
        (
            _0: ("Sue 1: goldfish: 9, cars: 0, samoyeds: 9"), vec![
                Sue::new(
                    1,
                    HashMap::from([
                        ("goldfish".to_string(), 9),
                        ("cars".to_string(), 0),
                        ("samoyeds".to_string(), 9),

                    ])
                )
            ]
            _1: ("Sue 387: cats: 0, perfumes: 5, akitas: 9"), vec![
                Sue::new(
                    387,
                    HashMap::from([
                        ("cats".to_string(), 0),
                        ("perfumes".to_string(), 5),
                        ("akitas".to_string(), 9),

                    ])
                )
            ]
        )
    }
}
