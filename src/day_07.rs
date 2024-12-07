use std::fs;

struct Equations {
    result: usize,
    numbers: Vec<usize>
}

enum Operation {
    Mul,
    Add,
    Pipe
}

impl Operation {
    pub fn equals_total(&self, eq: &Equations, index: usize, cur_total: usize, ops: &[Operation]) -> bool {
        let cur_num = eq.numbers.get(index);
        if let Some(&cur_num) = cur_num {
            ops
                .iter()
                .any(|op| op
                    .safe_apply(cur_total, cur_num)
                    .filter(|&applied| applied <= eq.result)
                    .map_or(false, |applied| op
                        .equals_total(eq, index + 1, applied, ops)
                    )
                )
        } else {
            cur_total == eq.result
        }
    }

    fn safe_apply(&self, a: usize, b: usize) -> Option<usize> {
        match self {
            Operation::Mul => a.checked_mul(b),
            Operation::Add => a.checked_add(b),
            Operation::Pipe => format!("{}{}", a, b).parse().ok()
        }
    }
}

impl Equations {
    fn is_solvable_with(&self, ops: &[Operation]) -> bool {
        ops.iter().any(|op| op.equals_total(self, 0, 0, ops))
    }
}

impl FromIterator<String> for Equations {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut items = iter.into_iter();
        let result = items
            .next()
            .expect("Expecting result in line")
            .parse()
            .expect("Expecting value to be parsable to usize!");
        let numbers = items
            .next()
            .expect("Expecting tokens after result!")
            .split_whitespace()
            .map(|token| token.parse().expect("Expecting token to be parsable to usize!"))
            .collect();
        Equations {
            result,
            numbers
        }
    }
}

// Main function
pub fn day_07() {
    let input_file = "./input/day_07.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> usize {
    parse_to_equations(input_file)
        .iter()
        .filter_map(|eq| eq
            .is_solvable_with(&[Operation::Mul, Operation::Add])
            .then_some(eq.result)
        )
        .sum()
}

// Puzzle 2 function
fn puzzle2(input_file: &str) -> usize {
    parse_to_equations(input_file)
        .iter()
        .filter_map(|eq| eq
            .is_solvable_with(&[Operation::Mul, Operation::Add, Operation::Pipe])
            .then_some(eq.result)
        )
        .sum()
}

fn parse_to_equations(path: &str) -> Vec<Equations> {
    fs::read_to_string(path)
        .expect("Failed to read input file")
        .lines()
        .map(|line| line.split(": ").map(String::from).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1("./input_test/day_07.txt"), 3749);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("./input_test/day_07.txt"), 11387);
    }
}
