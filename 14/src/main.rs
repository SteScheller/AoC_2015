use regex::Regex;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq)]
struct Reindeer {
    name: String,
    speed: u32,
    t_run: u32,
    t_rest: u32,
    t: u32,
    p: u32,
}

impl Reindeer {
    pub fn new(name: &str, speed: u32, t_run: u32, t_rest: u32) -> Self {
        Reindeer {
            name: String::from_str(name).unwrap(),
            speed,
            t_run,
            t_rest,
            t: 0,
            p: 0,
        }
    }

    pub fn compute_distance(&self, t_end: u32) -> u32 {
        let t_period = self.t_run + self.t_rest;
        let s_period = self.speed * self.t_run;
        let mut s = s_period * (t_end / t_period);
        s += core::cmp::min(t_end % t_period, self.t_run) * self.speed;
        s
    }

    pub fn advance(&mut self, dt: u32) {
        for _ in 0..dt {
            self.t += 1;
            if self.t <= self.t_run {
                self.p += self.speed;
            } else if self.t >= (self.t_run + self.t_rest) {
                self.t = 0;
            }
        }
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
    reindeers
        .into_iter()
        .map(|x| x.compute_distance(t_end))
        .max()
        .unwrap()
}

fn part_two(input: &str, t_end: u32) -> u32 {
    let mut reindeers = get_reindeers(input);
    let mut points = HashMap::new();
    for _ in 0..t_end {
        for r in reindeers.iter_mut() {
            r.advance(1);
        }

        let max_p = reindeers
            .iter()
            .max_by_key(|&r| r.p)
            .unwrap().p;

        for (i, r) in reindeers.iter().enumerate() {
            if r.p == max_p {
                *points.entry(i).or_insert(0) += 1;
            }
        }
        // TODO: all reindeers that share the lead should get a point
    }

    let (_, max_points) = points.iter().max_by_key(|&(_, v)| v).unwrap();
    *max_points
}

fn main() {
    let input = common::read_input("input.txt");
    println!("{}", part_one(&input, 2503));
    println!("{}", part_two(&input, 2503));
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

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_DATA, 1000), 689);
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
