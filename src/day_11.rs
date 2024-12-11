use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Stone {
    Zero,
    Split(usize, usize),
    Multiply(usize)
}

impl Stone {
    fn from(num: usize) -> Stone {
        if num == 0 {
            Stone::Zero
        } else {
            let num_digits = (num as f64).log10().floor() as usize + 1;
            if num_digits % 2 == 0 {
                let half_size = num_digits / 2;
                let divisor = 10_usize.pow(half_size as u32);
                Stone::Split(num / divisor, num % divisor)
            } else {
                Stone::Multiply(num)
            }
        }
    }

    fn step(&self) -> Vec<Stone> {
        match self {
            Stone::Zero => vec![Stone::from(1)],
            Stone::Split(l, r) => vec![Stone::from(*l), Stone::from(*r)],
            Stone::Multiply(m) => vec![Stone::from(*m * 2024)],
        }
    }

    fn step_mapped(&self, stone_paths: &mut HashMap<(Stone, usize), usize>, times: usize) -> usize {
        if times == 0 {
            stone_paths.insert((*self, 0), 1);
            return 1
        }

        if let Some(&stones) = stone_paths.get(&(*self, times)) {
            stones
        } else {
            for i in 0..times {
                let next = self
                    .step()
                    .iter()
                    .map(|stone| stone.step_mapped(stone_paths, i))
                    .sum();
                stone_paths.insert((*self, i + 1), next);
            }
            *stone_paths.get(&(*self, times)).unwrap()
        }
    }

    fn step_times(&self, times: usize) -> Vec<Stone> {
        let mut res = self.step();
        for _ in 1..times {
            res = res.iter().flat_map(|stone| stone.step()).collect();
        }
        res
    }
}

// Main function
pub fn day_11() {
    let input_file = "./input/day_11.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> usize {
    read_to_stones(input_file)
        .iter()
        .flat_map(|stone| stone.step_times(25))
        .count()
}

// Puzzle 2 function
fn puzzle2(input_file: &str) -> usize {
    let mut stone_paths: HashMap<(Stone, usize), usize> = HashMap::new();
    read_to_stones(input_file)
        .iter()
        .map(|stone| stone.step_mapped(&mut stone_paths, 75))
        .sum()
}

fn read_to_stones(input_file: &str) -> Vec<Stone> {
    fs::read_to_string(input_file)
        .expect("Failed to read input file")
        .split_whitespace()
        .map(|str| Stone::from(str.parse().expect("String must be usize!")))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stone_from() {
        let init_1 = Stone::from(125);
        assert_eq!(init_1, Stone::Multiply(125));
        let init_2 = Stone::from(17);
        assert_eq!(init_2, Stone::Split(1, 7));

        let step_1_1 = Stone::from(253000);
        assert_eq!(Stone::Split(253, 0), step_1_1);
        let step_1_2 = Stone::from(1);
        assert_eq!(Stone::Multiply(1), step_1_2);
        let step_1_3 = Stone::from(7);
        assert_eq!(Stone::Multiply(7), step_1_3);

        let step_2_1 = Stone::from(253);
        assert_eq!(Stone::Multiply(253), step_2_1);
        let step_2_2 = Stone::from(0);
        assert_eq!(Stone::Zero, step_2_2);
    }

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1("./input_test/day_11.txt"), 55312);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("./input_test/day_11.txt"), 55312);
    }
}
