struct LightGrid {
    width: usize,
    height: usize,
    arr: Vec<Vec<bool>>,
}

impl LightGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let arr = vec![vec![false; height]; width];
        Self { width, height, arr }
    }

    pub fn from_str(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut arr = Vec::new();
        for line in input.lines() {
            width = line.len();
            height += 1;
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c == '#');
            }
            arr.push(row);
        }
        Self { width, height, arr }
    }

    pub fn get_num_switched_on(&self) -> usize {
        self.arr.iter().flatten().filter(|x| **x).count()
    }

    fn simulate_step(&mut self) -> () {
        let current = self.arr.clone();
    }

    pub fn simulate_steps(&mut self, steps: usize) -> () {
        for _ in 0..steps {
            self.simulate_step();
        }
    }
}

fn part_one(input: &str, steps: usize) -> usize {
    let mut grid = LightGrid::from_str(input);
    grid.simulate_steps(steps);
    grid.get_num_switched_on()
}

fn part_two(input: &str) -> u32 {
    0
}

fn main() {
    let input = common::read_input("input.txt");
    println!("{}", part_one(&input, 100));
    println!("{}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_grid_construction() {
        let grid = LightGrid::from_str(
            ".#.#.#\n\
            ...##.\n\
            #....#\n\
            ..#...\n\
            #.#..#\n\
            ####.."
        );
        assert_eq!(grid.get_num_switched_on(), 15);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(
                ".#.#.#\n\
                ...##.\n\
                #....#\n\
                ..#...\n\
                #.#..#\n\
                ####..",
                4
            ),
            4
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("2,3,0,3,10,11,12,1,1,0,1,99,2,1,1,2"), 4);
    }
}
