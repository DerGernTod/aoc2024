use std::fs;

struct ClawMachine {
    a: (usize, usize),
    b: (usize, usize),
    price: (usize, usize)
}

impl ClawMachine {
    fn new(a: (usize, usize), b: (usize, usize), price: (usize, usize)) -> ClawMachine {
        ClawMachine{
            a,
            b,
            price
        }
    }
}

// Main function
pub fn day_13() {
    let input_file = "./input/day_13.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    0
}

// Puzzle 2 function
fn puzzle2(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1("./input_test/day_13.txt"), 0);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("./input_test/day_13.txt"), 0);
    }
}
