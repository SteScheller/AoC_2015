#[derive(Clone)]
struct LightGrid {
    arr: Vec<Vec<bool>>,
}

impl LightGrid {
    pub fn from_str(input: &str) -> Self {
        let mut arr = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c == '#');
            }
            arr.push(row);
        }
        Self { arr }
    }

    pub fn get_num_switched_on(&self) -> usize {
        self.arr.iter().flatten().filter(|x| **x).count()
    }

    fn get_num_neighbors_on(&self, x: usize, y: usize) -> usize {
        let mut result = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                if let Some(row) = self.arr.get((y as i32 + i) as usize) {
                    if let Some(element) = row.get((x as i32 + j) as usize) {
                        if *element {
                            result += 1;
                        }
                    }
                }
            }
        }
        result
    }

    fn simulate_step(&mut self) {
        let current = self.clone();
        for (y, row) in current.arr.iter().enumerate() {
            for (x, element) in row.iter().enumerate() {
                let num_neighbors = current.get_num_neighbors_on(x, y);
                match (element, num_neighbors) {
                    (true, 2) | (true, 3) => (),
                    (false, 3) => self.arr[y][x] = true,
                    _ => self.arr[y][x] = false,
                }
            }
        }
    }

    fn set_corners_on(&mut self) {
        let (x_max, y_max) = (self.arr[0].len() - 1, self.arr.len() - 1);
        self.arr[0][0] = true;
        self.arr[0][x_max] = true;
        self.arr[y_max][0] = true;
        self.arr[y_max][x_max] = true;
    }

    pub fn simulate_steps(&mut self, steps: usize, corners_stuck_on: bool) {
        if corners_stuck_on {
            self.set_corners_on()
        }
        for _ in 0..steps {
            self.simulate_step();
            if corners_stuck_on {
                self.set_corners_on()
            }
        }
    }
}

fn part_one(input: &str, steps: usize) -> usize {
    let mut grid = LightGrid::from_str(input);
    grid.simulate_steps(steps, false);
    grid.get_num_switched_on()
}

fn part_two(input: &str, steps: usize) -> usize {
    let mut grid = LightGrid::from_str(input);
    grid.simulate_steps(steps, true);
    grid.get_num_switched_on()
}

fn main() {
    let input = common::read_input("input.txt");
    println!("{}", part_one(&input, 100));
    println!("{}", part_two(&input, 100));
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
            ####..",
        );
        assert_eq!(grid.get_num_switched_on(), 15);
    }

    #[test]
    fn test_light_grid_get_num_neighbors_on() {
        let grid = LightGrid::from_str(
            ".#.#.#\n\
            ...##.\n\
            #....#\n\
            ..#...\n\
            #.#..#\n\
            ####..",
        );
        assert_eq!(grid.get_num_neighbors_on(0, 0), 1);
        assert_eq!(grid.get_num_neighbors_on(0, 1), 2);
        assert_eq!(grid.get_num_neighbors_on(1, 0), 0);
        assert_eq!(grid.get_num_neighbors_on(1, 1), 2);
        assert_eq!(grid.get_num_neighbors_on(2, 1), 3);
        assert_eq!(grid.get_num_neighbors_on(2, 2), 2);
        assert_eq!(grid.get_num_neighbors_on(5, 5), 1);
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
        assert_eq!(
            part_two(
                "##.#.#\n\
                ...##.\n\
                #....#\n\
                ..#...\n\
                #.#..#\n\
                ####.#",
                5
            ),
            17
        );
    }
}
