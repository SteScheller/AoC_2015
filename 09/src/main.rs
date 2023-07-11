use std::collections::{HashMap, HashSet};

use itertools::Itertools;
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
            length += distances.get(&(current, next)).unwrap();
            current = next;
        }
        if length < shortest_route {
            shortest_route = length;
        }
    }
    shortest_route
}

fn part_two(input: &str) -> u32 {
    let distances = get_distances(input);
    let locations = get_locations(input);
    let mut longest_route = 0;

    let routes = locations.iter().permutations(locations.len());
    for route in routes {
        let mut length = 0;
        let mut iter = route.into_iter();
        let mut current = iter.next().unwrap();
        while let Some(next) = iter.next() {
            length += distances.get(&(current, next)).unwrap();
            current = next;
        }
        if length > longest_route {
            longest_route = length;
        }
    }
    longest_route
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
        assert_eq!(
            part_two(
                "London to Dublin = 464\n\
            London to Belfast = 518\n\
            Dublin to Belfast = 141"
            ),
            982
        );
    }
}
