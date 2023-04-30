use std::fs;

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}

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

fn main() {
    let input = read_input("input.txt");
    println!("{}", part_one(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("2x3x4"), 58);
        assert_eq!(part_one("1x1x10"), 43);
    }
}
