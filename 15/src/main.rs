use itertools::iproduct;
use regex::Regex;
use std::{collections::HashMap, hash::Hash, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    pub fn new(
        name: &str,
        capacity: i32,
        durability: i32,
        flavor: i32,
        texture: i32,
        calories: i32,
    ) -> Self {
        Ingredient {
            name: String::from_str(name).unwrap(),
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

type Recipe<'a> = HashMap<&'a Ingredient, u32>;

fn get_ingredients(input: &str) -> Vec<Ingredient> {
    let re = Regex::new(
        r"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)"
    ).unwrap();

    let mut ingredients = Vec::new();
    for line in input.lines() {
        if let Some(c) = re.captures(line) {
            ingredients.push(Ingredient::new(
                c.get(1).unwrap().as_str(),
                c.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                c.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                c.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                c.get(5).unwrap().as_str().parse::<i32>().unwrap(),
                c.get(6).unwrap().as_str().parse::<i32>().unwrap(),
            ));
        }
    }
    ingredients
}

fn calc_score(recipe: &Recipe) -> u32 {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;

    for (ingredient, n) in recipe {
        capacity += *n as i32 * ingredient.capacity;
        durability += *n as i32 * ingredient.durability;
        flavor += *n as i32 * ingredient.flavor;
        texture += *n as i32 * ingredient.texture;
    }

    let score = [capacity, durability, flavor, texture]
        .into_iter()
        .map(|x| std::cmp::max(x, 1))
        .fold(1, std::ops::Mul::mul);
    score as u32
}

fn get_partitions(n: u32, k: u32) -> Vec<std::ops::RangeInclusive<u32>> {
    let mut partitions = Vec::new();

    for _ in 0..k {
        partitions.push(0..=n)
    }
    partitions
}

fn part_one(input: &str, sum_spoons: u32) -> u32 {
    let ingredients = get_ingredients(input);
    let r = 0..=sum_spoons;
    let mut max_score = 0;

    for (i, j, k, l) in iproduct!(r.clone(), r.clone(), r.clone(), r) {
        if (i + j + k + l) > sum_spoons {
            continue;
        }
        let mut recipe = HashMap::new();
        recipe.insert(&ingredients[0], i);
        recipe.insert(&ingredients[1], j);
        recipe.insert(&ingredients[2], k);
        recipe.insert(&ingredients[3], l);
        max_score = std::cmp::max(calc_score(&recipe), max_score);
    }
    max_score
}

fn part_two(_input: &str, _sum_spoons: u32) -> u32 {
    0
}

fn main() {
    let input = common::read_input("input.txt");
    println!("{}", part_one(&input, 100));
    println!("{}", part_two(&input, 100));
}

#[cfg(test)]
mod tests {
    use common::{parametrized_tests, parametrized_tests_single};
    use indoc::indoc;

    use super::*;

    const TEST_DATA: &str = indoc! {"
        Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
        Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
    "};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_DATA, 100), 62842880);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_DATA, 1000), 689);
    }

    #[test]
    fn test_calc_score() {
        let mut recipe = HashMap::new();
        let ingredients = get_ingredients(TEST_DATA);
        recipe.insert(&ingredients[0], 44); // butterscotch
        recipe.insert(&ingredients[1], 56); // cinnamon
        assert_eq!(calc_score(&recipe), 62842880);
    }

    parametrized_tests! {
        test_get_ingredients_butterscotch: (
            get_ingredients,
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8",
            vec![Ingredient::new("Butterscotch", -1, -2, 6, 3, 8)]),
       test_get_ingredients_cinnamon: (
            get_ingredients,
            "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
            vec![Ingredient::new("Cinnamon", 2, 3, -2, -1, 3)]),
    }

    parametrized_tests_single! {
        get_partitions,
        (
            _0: (42, 0), Vec::<std::ops::RangeInclusive<u32>>::new()
            _1: (3, 1), Vec::from([0..=3])
            _2: (3, 2), Vec::from([0..=3, 0..=3])
            _3: (3, 3), Vec::from([0..=3, 0..=3, 0..=3])
        )
    }
}
