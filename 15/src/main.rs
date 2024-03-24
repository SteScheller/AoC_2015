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

fn get_partitions(n: u32, k: u32) -> Vec<Vec<u32>> {
    let mut partitions = Vec::new();

    match k {
        0 => (),
        1 => partitions.push(vec![n]),
        _ => {
            for x in 0..=n {
                let sub_partitions = get_partitions(n - x, k - 1);
                for sp in sub_partitions {
                    let mut p = vec![x];
                    p.extend(sp);
                    partitions.push(p);
                }
            }
        }
    }

    partitions
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

fn part_one(input: &str, sum_spoons: u32) -> u32 {
    let ingredients = get_ingredients(input);
    let num_ingredients = ingredients.len() as u32;
    let partitions = get_partitions(sum_spoons, num_ingredients);
    let mut max_score = 0;

    for p in partitions.into_iter() {
        let mut recipe = HashMap::new();
        for (i, n_ingredient) in p.into_iter().enumerate() {
            recipe.insert(&ingredients[i], n_ingredient);
        }
        max_score = std::cmp::max(calc_score(&recipe), max_score);
    }
    max_score
}

fn part_two(input: &str, sum_spoons: u32) -> u32 {
    let ingredients = get_ingredients(input);
    let num_ingredients = ingredients.len() as u32;
    let partitions = get_partitions(sum_spoons, num_ingredients);
    let mut max_score = 0;

    for p in partitions.into_iter() {
        let mut recipe = HashMap::new();
        for (i, n_ingredient) in p.into_iter().enumerate() {
            recipe.insert(&ingredients[i], n_ingredient);
        }
        let score = calc_score(&recipe);
        let calories = recipe
            .into_iter()
            .map(|x| x.0.calories as u32 * x.1)
            .fold(0, std::ops::Add::add);
        if calories == 500 {
            max_score = std::cmp::max(score, max_score);
        }
    }
    max_score
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
            _0: (42, 0), Vec::<Vec<u32>>::new()
            _1: (3, 1), vec![vec![3]]
            _2: (3, 2), vec![vec![0, 3], vec![1,2], vec![2,1], vec![3, 0]]
            _3: (3, 3), vec![
                vec![0, 0, 3], vec![0, 1, 2], vec![0, 2, 1], vec![0, 3, 0],
                vec![1, 0, 2], vec![1, 1, 1], vec![1, 2, 0],
                vec![2, 0, 1], vec![2, 1, 0],
                vec![3, 0, 0],
                ]
        )
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_DATA, 100), 62842880);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_DATA, 100), 57600000);
    }
}
