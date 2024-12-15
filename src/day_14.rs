use std::{cmp::Ordering, collections::HashMap, fs, io, ops::{Add, Div, Mul, Rem}};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2D(isize, isize);

impl Mul<isize> for Vec2D {
    type Output = Vec2D;

    fn mul(self, rhs: isize) -> Self::Output {
        Vec2D(self.0 * rhs, self.1 * rhs)
    }
}

impl Add for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Rem for Vec2D {
    type Output = Vec2D;

    fn rem(self, rhs: Self) -> Self::Output {
        Vec2D(self.0 % rhs.0, self.1 % rhs.1)
    }
}

impl Div<isize> for Vec2D {
    type Output = Vec2D;

    fn div(self, rhs: isize) -> Self::Output {
        Vec2D(self.0 / rhs, self.1 / rhs)
    }
}

impl FromIterator<String> for Vec2D {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let x = iter.next().expect("Expecting first coord to be value").parse().expect("Expecting first coord to be isize");
        let y = iter.next().expect("Expecting second coord to be value").parse().expect("Expecting second coord to be isize");
        Vec2D(x, y)
    }
}

struct Robot {
    position: Vec2D,
    velocity: Vec2D,
    limit: Vec2D
}

impl Robot {
    fn do_move(&self, iterations: isize) -> Robot {
        let mut new_pos = (self.position + self.velocity * iterations) % self.limit;
        if new_pos.0 < 0 {
            new_pos.0 += self.limit.0;
        }
        if new_pos.1 < 0 {
            new_pos.1 += self.limit.1;
        }
        Robot {
            position: new_pos,
            limit: self.limit,
            velocity: self.velocity
        }
    }

    fn step(&mut self) {
        let mut new_pos = (self.position + self.velocity) % self.limit;
        if new_pos.0 < 0 {
            new_pos.0 += self.limit.0;
        }
        if new_pos.1 < 0 {
            new_pos.1 += self.limit.1;
        }
        self.position = new_pos
    }

    fn is_left(&self) -> bool {
        let edge = self.limit.0 / 2;
        self.position.0 < edge
    }

    fn is_top(&self) -> bool {
        let edge = self.limit.1 / 2;
        self.position.1 < edge
    }

    fn is_on_edge(&self) -> bool {
        let edge = self.limit / 2;
        self.position.0 == edge.0 || self.position.1 == edge.1
    }
}

// Main function
pub fn day_14() {
    let input_file = "./input/day_14.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> usize {
    let (lefts, rights): (Vec<Robot>, Vec<Robot>) = read_to_bots(input_file)
        .into_iter()
        .map(|bot| bot.do_move(100))
        .filter(|bot| !bot.is_on_edge())
        .partition(|bot| bot.is_left());
    let (left_tops, left_bottoms): (Vec<Robot>, Vec<Robot>) = lefts.into_iter().partition(|bot| bot.is_top());
    let (right_tops, right_bottoms): (Vec<Robot>, Vec<Robot>) = rights.into_iter().partition(|bot| bot.is_top());
    left_bottoms.len() * left_tops.len() * right_bottoms.len() * right_tops.len()
}

fn read_to_bots(input_file: &str) -> Vec<Robot> {
    fs::read_to_string(input_file)
        .expect("Failed to read input file")
        .lines()
        .map(|line| {
            let mut vecs: Vec<Vec2D> = line
                .split_whitespace()
                .map(|vec| vec
                    .replace("p=", "")
                    .replace("v=", "")
                    .split(",")
                    .map(|str| String::from(str))
                    .collect()
                )
                .collect();
            Robot {
                position: vecs.swap_remove(0),
                velocity: vecs.swap_remove(0),
                limit: Vec2D(101, 103)//11, 7)
            }
        })
        .collect()
}

// Puzzle 2 function
fn puzzle2(input_file: &str) -> usize {
    let mut bots = read_to_bots(input_file);
    let mut iteration = 0;
    loop {
        bots.iter_mut().for_each(|bot| bot.step());
        let map: HashMap<Vec2D, usize> = bots
            .iter()
            .fold(HashMap::new(), |mut map, bot| {
                let entry = map.entry(bot.position).or_default();
                *entry += 1;
                map
            });
        print_map(&map);
        println!("Iteration {} complete. x to exit: ", iteration);
        let mut input = String::new(); // Create a mutable String to store the input
        io::stdin()
            .read_line(&mut input) // Read input and store it in the `input` variable
            .expect("Failed to read line");

        if input == "x\n" {
            break;
        }
        iteration += 1;
    }
    iteration
}

fn print_map(map: &HashMap<Vec2D, usize>) {
    for y in 0..103 {
        for x in 0..101 {
            if let Some(num) = map.get(&Vec2D(x, y)) {
                print!("{}", num);
            } else {
                print!(".");
            }
        }
        println!();

    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1("./input_test/day_14.txt"), 12);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("./input_test/day_14.txt"), 0);
    }
}
