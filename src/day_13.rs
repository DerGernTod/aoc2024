use std::{cmp::Ordering, fs, ops::{Add, Div, Mul, Rem}, usize};

#[derive(Debug)]
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

    fn solve(&self) -> Option<usize> {
        let ClawMachine{ a, b, prize } = self;
        let det = a.0 * b.1 - a.1 * b.0;
        if det == 0 {
            return None;
        }

        let num_a = prize.0 * b.1 - prize.1 * b.0;
        let num_b = a.0 * prize.1 - a.1 * prize.0;
        if num_a % det != 0 || num_b % det != 0 {
            return None;
        }

        let int_a = num_a / det;
        let int_b = num_b / det;
    
        Some(int_a as usize * 3 + int_b as usize)
    }

}

impl FromIterator<String> for ClawMachine {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut str_iter = iter.into_iter();

        let str_a = str_iter
            .next()
            .expect("Expecting Button A");
        let mut str_a = str_a
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

        let str_b = str_iter
            .next()
            .expect("Expecting Button B");
        let mut str_b = str_b
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

        let str_prize = str_iter
            .next()
            .expect("Expecting prize");
        let mut str_prize = str_prize
            .split_whitespace()
            .skip(1)
            .map(|str| 
                str
                    .replace("X=", "")
                    .replace("Y=", "")
                    .replace(",", "")
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
    read_claw_machines(input_file)
        .iter()
        .filter_map(|machine| machine.solve())
        .sum()
}

fn read_claw_machines(input_file: &str) -> Vec<ClawMachine> {
    fs::read_to_string(input_file)
        .expect("Failed to read input file")
        .split("\n\n")
        .map(|str| str
            .lines()
            .map(|str| String::from(str)).collect())
        .collect()
}

// Puzzle 2 function
fn puzzle2(input_file: &str) -> usize {
    read_claw_machines(input_file)
        .into_iter()
        .map(|machine| ClawMachine {
            a: machine.a,
            b: machine.b,
            prize: (machine.prize.0 + 10000000000000, machine.prize.1 + 10000000000000)
        })
        .filter_map(|machine| machine.solve())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_solve() {
        let machine = ClawMachine {
            a: (94, 34),
            b: (22, 67),
            prize: (8400, 5400)
        };
        assert_eq!(machine.solve(), Some(280));

        let machine = ClawMachine {
            a: (26, 66),
            b: (67, 21),
            prize: (12748, 12176)
        };
        assert_eq!(machine.solve(), None);

        let machine = ClawMachine {
            a: (17, 86),
            b: (84, 37),
            prize: (7870, 6450)
        };
        assert_eq!(machine.solve(), Some(200));

        let machine = ClawMachine {
            a: (69, 23),
            b: (27, 71),
            prize: (18641, 10279)
        };
        assert_eq!(machine.solve(), None);
    }

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1("./input_test/day_13.txt"), 480);
    }

    #[test]
    fn test_puzzle2() {
        // this breaks
        assert_eq!(puzzle2("./input_test/day_13.txt"), 480);
    }
}
