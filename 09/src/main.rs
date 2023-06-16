use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use itertools::Itertools;
use num::Bounded;
use regex::Regex;

use common;

fn get_distances(input: &str) -> HashMap<(&str, &str), u32> {
    let re_location_distances = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    let mut distances = HashMap::new();
    for line in input.lines() {
        if let Some(c) = re_location_distances.captures(line) {
            let location_a = c.get(1).unwrap().as_str();
            let location_b = c.get(2).unwrap().as_str();
            distances.insert(
                (location_a, location_b),
                c.get(3).unwrap().as_str().parse::<u32>().unwrap(),
            );
            distances.insert(
                (location_b, location_a),
                c.get(3).unwrap().as_str().parse::<u32>().unwrap(),
            );
        }
    }
    distances
}

fn get_locations(input: &str) -> HashSet<&str> {
    let re_locations = Regex::new(r"(\w+) to (\w+)").unwrap();
    let mut locations = HashSet::new();
    for line in input.lines() {
        if let Some(c) = re_locations.captures(line) {
            let location_a = c.get(1).unwrap().as_str();
            let location_b = c.get(2).unwrap().as_str();
            locations.insert(location_a);
            locations.insert(location_b);
        }
    }
    locations
}

fn compute_shortest_path<T>(
    origin: &str,
    destination: &str,
    locations: &HashSet<&str>,
    distances: &HashMap<(&str, &str), T>,
) -> T
where
    T: Bounded + Copy + Add<Output = T>,
    <T as Add>::Output: PartialOrd<T>,
{
    let mut shortest_distances = distances.clone();
    let mut unvisited = locations.clone();
    unvisited.remove(origin);

    for current in unvisited.drain() {
        for neighbor in locations.iter() {
            if *neighbor == current {
                continue;
            }
            let old = *shortest_distances
                .get(&(origin, *neighbor))
                .unwrap_or(&T::max_value());
            let new = *distances.get(&(origin, current)).unwrap()
                + *distances.get(&(current, *neighbor)).unwrap();
            if new < old {
                shortest_distances.insert((origin, *neighbor), new);
            }
        }
    }

    *shortest_distances
        .get(&(origin, destination))
        .unwrap_or(&T::max_value())
}

fn part_one(input: &str) -> u32 {
    let distances = get_distances(input);
    let locations = get_locations(input);
    let mut shortest_route = u32::MAX;

    let routes = locations.iter().permutations(locations.len());
    for route in routes {
        let mut length = 0;
        let mut iter = route.into_iter();
        let mut current = iter.next().unwrap();
        while let Some(next) = iter.next() {
            length += compute_shortest_path(*current, *next, &locations, &distances);
            current = next;
        }
        if length < shortest_route {
            shortest_route = length;
        }
    }
    shortest_route
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

    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(
                "London to Dublin = 464\n\
            London to Belfast = 518\n\
            Dublin to Belfast = 141"
            ),
            605
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("2x3x4"), 0);
    }
}
