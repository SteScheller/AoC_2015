use std::collections::HashMap;

fn part_one(input: &str) -> usize {
    let (mut x, mut y) = (0, 0);
    let mut visits = HashMap::new();

    visits.insert([x, y], 1);

    for c in input.chars() {
        match c {
            '<' => x -= 1,
            '>' => x += 1,
            '^' => y -= 1,
            'v' => y += 1,
            _ => continue,
        }
        visits.entry([x, y]).and_modify(|x| *x += 1).or_insert(1);
    }

    visits.len()
}

fn part_two(input: &str) -> usize {
    let (mut xs, mut ys) = ([0, 0], [0, 0]);
    let mut visits = HashMap::new();

    visits.insert([xs[0], ys[0]], 1);

    for (i, c) in input.char_indices().step_by(2) {
        let directions = match input.get(i + 1..=i + 1) {
            Some(next_c) => [c, next_c.chars().next().unwrap()],
            _ => continue,
        };
        for i in 0..2 {
            match directions[i] {
                '<' => xs[i] -= 1,
                '>' => xs[i] += 1,
                '^' => ys[i] -= 1,
                'v' => ys[i] += 1,
                _ => continue,
            }
            visits
                .entry([xs[i], ys[i]])
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
    }

    visits.len()
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
        assert_eq!(part_one(">"), 2);
        assert_eq!(part_one("^>v<"), 4);
        assert_eq!(part_one("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("^v"), 3);
        assert_eq!(part_two("^>v<"), 3);
        assert_eq!(part_two("^v^v^v^v^v"), 11);
    }
}
