fn part_one(_input: &str) -> usize {
    0
}
fn part_two(_input: &str) -> usize {
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
                ".#.#.#\n\
                ...##.\n\
                #....#\n\
                ..#...\n\
                #.#..#\n\
                ####.."
            ),
            4
        );
    }
}
