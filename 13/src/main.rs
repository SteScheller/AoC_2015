use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct Relation {
    p_a: String,
    p_b: String,
    w: i8,
}

impl Relation {
    pub fn new(p_a: &str, p_b: &str, w: i8) -> Self {
        Relation {
            p_a: String::from_str(p_a).unwrap(),
            p_b: String::from_str(p_b).unwrap(),
            w,
        }
    }
}

fn get_relations(input: &str) -> Vec<Relation> {
    let re = Regex::new(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)")
        .unwrap();
    let mut relations = Vec::new();
    for line in input.lines() {
        if let Some(c) = re.captures(line) {
            let sign = match c.get(2).unwrap().as_str() {
                "lose" => -1,
                _ => 1,
            };
            relations.push(Relation::new(
                c.get(1).unwrap().as_str(),
                c.get(4).unwrap().as_str(),
                c.get(3).unwrap().as_str().parse::<i8>().unwrap() * sign,
            ))
        }
    }
    relations
}

fn get_persons(relations: &Vec<Relation>) -> Vec<String> {
    let mut persons = HashSet::new();
    for r in relations {
        if !persons.contains(&r.p_a) {
            persons.insert(r.p_a.clone());
        }
        if !persons.contains(&r.p_b) {
            persons.insert(r.p_b.clone());
        }
    }
    persons.into_iter().collect()
}

fn get_relation_map(relations: &Vec<Relation>) -> HashMap<(&String, &String), i8> {
    let mut relation_map = HashMap::new();
    for r in relations {
        relation_map.insert((&r.p_a, &r.p_b), r.w);
    }
    relation_map
}

fn part_one(input: &str) -> i32 {
    let relations = get_relations(input);
    let persons = get_persons(&relations);
    let relation_map = get_relation_map(&relations);
    let permutations = persons.iter().permutations(persons.len());

    let mut max_happiness = 0;
    for p in permutations {
        let mut happiness = 0;
        for pair in p.into_iter().tuple_windows() {
            if let Some(value) = relation_map.get(&pair) {
                happiness += *value as i32;
            }
        }
        max_happiness = std::cmp::max(happiness, max_happiness);
    }
    max_happiness
}

fn part_two(input: &str) -> i32 {
    let relations = get_relations(input);
    0
}

fn main() {
    let input = common::read_input("input.txt");
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use common::parametrized_tests;
    use indoc::indoc;

    use super::*;

    const TEST_DATA: &str = indoc! {"
        Alice would gain 54 happiness units by sitting next to Bob.
        Alice would lose 79 happiness units by sitting next to Carol.
        Alice would lose 2 happiness units by sitting next to David.
        Bob would gain 83 happiness units by sitting next to Alice.
        Bob would lose 7 happiness units by sitting next to Carol.
        Bob would lose 63 happiness units by sitting next to David.
        Carol would lose 62 happiness units by sitting next to Alice.
        Carol would gain 60 happiness units by sitting next to Bob.
        Carol would gain 55 happiness units by sitting next to David.
        David would gain 46 happiness units by sitting next to Alice.
        David would lose 7 happiness units by sitting next to Bob.
        David would gain 41 happiness units by sitting next to Carol.
    "};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_DATA), 330);
    }

    #[test]
    fn test_get_persons() {
        let mut persons = get_persons(&get_relations(TEST_DATA));
        persons.sort();
        assert_eq!(persons, vec!["Alice", "Bob", "Carol", "David"]);
    }

    parametrized_tests! {
        rel_0: (
            get_relations,
            "Alice would gain 54 happiness units by sitting next to Bob.",
            vec![Relation::new("Alice", "Bob", 54)]),
        rel_1: (
            get_relations,
            "Bob would lose 7 happiness units by sitting next to Carol.",
            vec![Relation::new("Bob", "Carol", -7)]),
    }
}
