use std::fs;

struct ClawMachine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize)
}

impl ClawMachine {
    fn new(a: (usize, usize), b: (usize, usize), prize: (usize, usize)) -> ClawMachine {
        ClawMachine{
            a,
            b,
            prize
        }
    }
}

impl FromIterator<String> for ClawMachine {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut str_iter = iter.into_iter();

        let mut str_a = str_iter
            .next()
            .expect("Expecting Button A")
            .split_whitespace()
            .skip(2)
            .map(|str| str
                .replace(",", "")
                .replace("X+", "")
                .replace("Y+", ""));
        let a = (
            str_a.next().expect("Expecting X for A").parse().expect("Expecting X for A to be usize"),
            str_a.next().expect("Expecting Y for A").parse().expect("Expecting Y for A to be usize")
        );

        let mut str_b = str_iter
            .next()
            .expect("Expecting Button B")
            .split_whitespace()
            .skip(2)
            .map(|str| str
                .replace(",", "")
                .replace("X+", "")
                .replace("Y+", ""));
        let b = (
            str_b.next().expect("Expecting X for B").parse().expect("Expecting X for B to be usize"),
            str_b.next().expect("Expecting Y for B").parse().expect("Expecting Y for B to be usize")
        );

        let mut str_prize = str_iter
            .next()
            .expect("Expecting prize")
            .split_whitespace()
            .skip(1)
            .map(|str| 
                str
                    .replace("X=", "")
                    .replace("Y=", "")
            );
        let prize = (
            str_prize.next().expect("Expecting X for Prize").parse().expect("Expecting X for Prize to be usize"),
            str_prize.next().expect("Expecting Y for Prize").parse().expect("Expecting Y for Prize to be usize")
        );
        ClawMachine::new(a, b, prize)
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
        assert_eq!(puzzle1("./input_test/day_13.txt"), 480);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("./input_test/day_13.txt"), 0);
    }
}
