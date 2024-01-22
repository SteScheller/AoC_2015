use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Reindeer {
    name: String,
    speed: u32,
    t_run: u32,
    t_rest: u32,
}

impl Reindeer {
    pub fn new(name: &str, speed: u32, t_run: u32, t_rest: u32) -> Self {
        Reindeer {
            name: String::from_str(name).unwrap(),
            speed,
            t_run,
            t_rest,
        }
    }

    pub fn compute_distance(t_end: u32) -> u32 {
        0
    }
}

fn get_reindeers(input: &str) -> Vec<Reindeer> {
    let re = Regex::new(
        r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();
    let mut reindeers = Vec::new();
    for line in input.lines() {
        if let Some(c) = re.captures(line) {
            reindeers.push(Reindeer::new(
                c.get(1).unwrap().as_str(),
                c.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                c.get(3).unwrap().as_str().parse::<u32>().unwrap(),
                c.get(4).unwrap().as_str().parse::<u32>().unwrap(),
            ));
        }
    }
    reindeers
}

fn part_one(input: &str, t_end: u32) -> u32 {
    let reindeers = get_reindeers(input);
    0
}

fn part_two(_input: &str) -> u32 {
    0
}

fn main() {
    let input = common::read_input("input.txt");
    println!("{}", part_one(&input, 2503));
    println!("{}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use common::parametrized_tests;
    use indoc::indoc;

    use super::*;

    const TEST_DATA: &str = indoc! {"
        Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
    "};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_DATA, 1000), 1120);
    }

    parametrized_tests! {
        reindeer_0: (
            get_reindeers,
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
            vec![Reindeer::new("Comet", 14, 10, 127)]),
        reindeer_1: (
            get_reindeers,
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
            vec![Reindeer::new("Dancer", 16, 11, 162)]),
    }
}
