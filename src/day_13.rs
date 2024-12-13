use std::{cmp::Ordering, fs, ops::{Add, Mul, Rem}};

#[derive(PartialEq, Eq, Ord, Clone, Copy, Debug)]
struct Point(usize, usize);

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 < other.0 && self.1 < other.1 {
            Some(Ordering::Less)
        } else if self.0 > other.0 && self.1 > other.1 {
            Some(Ordering::Greater)
        } else if self.0 == other.0 && self.1 == other.1 {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

impl Rem for Point {
    type Output = Point;

    fn rem(self, rhs: Self) -> Self::Output {
        Point(self.0 % rhs.0, self.1 % rhs.1)
    }
}

impl Mul<usize> for Point {
    type Output = Point;

    fn mul(self, rhs: usize) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}


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

    fn solve_high(&self) -> Option<usize> {
        let prize = Point(self.prize.0, self.prize.1) * 10000000000000;
        let a = Point(self.a.0, self.a.1);
        let b = Point(self.b.0, self.b.1);
        None
    }

    fn solve(&self) -> Option<usize> {
        // a cost 3, b cost 1
        let prize = Point(self.prize.0, self.prize.1);
        let a = Point(self.a.0, self.a.1);
        let b = Point(self.b.0, self.b.1);

        let mut min_cost: Option<usize> = None;

        for num_a in 0..=100 {
            for num_b in 0..=100 {
                let sum = a * num_a + b * num_b;
                if sum > prize {
                    break;
                } else if sum == prize {
                    let cost = num_a * 3 + num_b;
                    min_cost = min_cost
                        .filter(|&cur_min| cur_min < cost)
                        .or(Some(cost));
                    break;
                }
            }
        }
        min_cost
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
        .iter()
        .filter_map(|machine| machine.solve_high())
        .sum()
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
