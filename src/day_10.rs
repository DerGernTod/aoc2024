use std::{collections::{HashMap, HashSet}, fs, ops::{Add, Sub}};

#[derive(PartialEq, PartialOrd, Eq, Hash, Copy, Clone)]
struct Point(usize, usize);

impl Point {
    fn get_surroundings(&self) -> Vec<Point> {
        [
            self.checked_add(&Point(0, 1)),
            self.checked_add(&Point(1, 0)),
            self.checked_sub(&Point(0, 1)),
            self.checked_sub(&Point(1, 0))
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn checked_add(&self, rhs: &Point) -> Option<Point> {
        let x = self.0.checked_add(rhs.0)?;
        let y = self.1.checked_add(rhs.1)?;
        Some(Point(x, y))
    }

    fn checked_sub(&self, rhs: &Point) -> Option<Point> {
        let x = self.0.checked_sub(rhs.0)?;
        let y = self.1.checked_sub(rhs.1)?;
        Some(Point(x, y))
    }
}

// Main function
pub fn day_10() {
    let input_file = "./input/day_10.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> usize {
    let (nines, map) = read_map(input_file);
    nines
        .iter()
        .flat_map(|nine_coords| count_surrounding_lowers(9, nine_coords, &map))
        .count()
}

fn read_map(input_file: &str) -> (Vec<Point>, HashMap<Point, u32>) {
    fs::read_to_string(input_file)
        .expect("Failed to read input file")
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| 
            line
                .chars()
                .map(|char| 
                    char
                        .to_digit(10)
                        .expect("Expecting char to be digit!")
                )
                .enumerate()
                .map(move |(x, height)| (Point(x, y), height))
        )
        .fold((vec![], HashMap::new()), |(mut nines, mut map), (coords, height)| {
            if height == 9 {
                nines.push(coords);
            }
            map.insert(coords, height);
            (nines, map)
        })
}

fn count_surrounding_lowers(cur_height: u32, cur_coords: &Point, map: &HashMap<Point, u32>) -> HashSet<Point> {
    if cur_height == 0 {
        return HashSet::from([*cur_coords]);
    }
    cur_coords
        .get_surroundings()
        .iter()
        .filter_map(|coord| 
            map
                .get(coord)
                .filter(|&&height| cur_height - 1 == height)
                .map(|&height| count_surrounding_lowers(height, coord, map))
        )
        .flatten()
        .collect()
}

// Puzzle 2 function
fn puzzle2(input_file: &str) -> usize {
    let (nines, map) = read_map(input_file);
    nines
        .iter()
        .map(|nine_coords| count_surrounding_lowers_unique(9, nine_coords, &map))
        .sum()
}

fn count_surrounding_lowers_unique(cur_height: u32, cur_coords: &Point, map: &HashMap<Point, u32>) -> usize {
    if cur_height == 0 {
        return 1;
    }
    cur_coords
        .get_surroundings()
        .iter()
        .filter_map(|coord| 
            map
                .get(coord)
                .filter(|&&height| cur_height - 1 == height)
                .map(|&height| count_surrounding_lowers_unique(height, coord, map))
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1("./input_test/day_10.txt"), 36);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("./input_test/day_10.txt"), 81);
    }
}
