use std::collections::{HashMap, HashSet};

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

fn find_closest_destination<'a>(
    origin: &str,
    destinations: &HashSet<&'a str>,
    distances: &HashMap<(&str, &str), u32>,
) -> &'a str {
    let mut min_distance = u32::MAX;
    let mut closest_destination = None;
    for dest in destinations.iter() {
        println!("\t{}", *dest);
        let distance = distances.get(&(origin, *dest)).unwrap();
        if *distance <= min_distance {
            closest_destination = Some(*dest);
            min_distance = *distance;
        }
    }

    closest_destination.unwrap()
}

fn compute_shortest_paths(
    locations: HashSet<&str>,
    distances: &HashMap<(&str, &str), u32>,
) -> HashMap<(&str, &str), u32> {
    let mut paths = distances.clone();

    paths
}

fn part_one(input: &str) -> u32 {
    let mut shortest_route = 0;
    let distances = get_distances(input);
    let mut locations = get_locations(input);
    let mut origin;
    {
        let mut drain = locations.drain();
        origin = drain.next().unwrap();
        locations = HashSet::from_iter(drain);
    }

    println!("{origin}");
    while !locations.is_empty() {
        let destination = find_closest_destination(origin, &locations, &distances);
        let key = (origin, destination);
        shortest_route += distances.get(&key).unwrap();
        println!("{origin} {destination}");
        origin = destination;
        locations.remove(destination);
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
        assert_eq!(part_two("2x3x4"), 34);
    }
}
