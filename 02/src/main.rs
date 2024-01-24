fn part_one(input: &str) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        if let Ok([l, w, h]) = TryInto::<[u32; 3]>::try_into(
            line.split('x')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        ) {
            let [front, side, top] = [l * w, w * h, h * l];
            result += 2 * (l * w + w * h + h * l);
            result += [front, side, top].iter().min().unwrap();
        }
    }

    result
}

fn part_two(input: &str) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        if let Ok(mut dimensions) = TryInto::<[u32; 3]>::try_into(
            line.split('x')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        ) {
            dimensions.sort();
            let [a, b, _] = dimensions;
            result += 2 * (a + b);
            result += dimensions.iter().product::<u32>();
        }
    }

    result
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
        assert_eq!(part_one("2x3x4"), 58);
        assert_eq!(part_one("1x1x10"), 43);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("2x3x4"), 34);
        assert_eq!(part_two("1x1x10"), 14);
    }
}
