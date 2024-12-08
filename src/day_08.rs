use std::{collections::{HashMap, HashSet}, fmt::Display, fs};

#[derive(Eq, Hash, PartialEq, Clone, Debug, Copy)]
struct Point(i32, i32);

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl Point {
    fn safe_substract(&self, other: &Point) -> Option<Point> {
        let x = self.0.checked_sub(other.0)?;
        let y = self.1.checked_sub(other.1)?;
        Some(Point(x, y))
    }

    fn safe_add(&self, other: Option<Point>) -> Option<Point> {
        other.and_then(|other| {
            let x = self.0.checked_add(other.0)?;
            let y = self.1.checked_add(other.1)?;
            Some(Point(x, y))
        })
    }

    fn is_within_limits(&self, max_x: i32, max_y: i32) -> bool {
        self.0 >= 0 && self.1 >= 0 && self.0 < max_x && self.1 < max_y
    }
}

// Main function
pub fn day_08() {
    let input_file = "./input/day_08.txt";
    
    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> usize {
    let (map, max_x, max_y) = read_map(input_file);
    let found_coords = map.iter()
        .fold(HashSet::new(), |antinodes, (_, coords)| {
            calc_coord_pairs(coords)
                .flat_map(|(coord1, coord2)| [
                    coord1.safe_add(coord1.safe_substract(coord2)),
                    coord2.safe_add(coord2.safe_substract(coord1))
                ])
                .flatten()
                .fold(antinodes, |mut antinodes, coord| { 
                    antinodes.insert(coord);
                    antinodes
                })
    });
    found_coords
        .iter()
        .filter(|coords| coords.is_within_limits(max_x, max_y))
        .count()
}

// Puzzle 2 function
fn puzzle2(input_file: &str) -> usize {
    let (map, max_x, max_y) = read_map(input_file);
    let found_coords = map.iter()
        .fold(HashSet::new(), |antinodes, (_, coords)| {
            calc_coord_pairs(coords)
                .flat_map(|(coord1, coord2)|
                    find_antinodes_for_coord_pair(coord1, coord2, max_x, max_y)
                )
                .fold(antinodes, |mut antinodes, coord| {
                    antinodes.insert(coord);
                    antinodes
                })
        });
    
    found_coords
        .iter()
        .filter(|coords| coords.is_within_limits(max_x, max_y))
        .count()
}

fn read_map(input_file: &str) -> (HashMap<char, Vec<Point>>, i32, i32) {
    let input = fs::read_to_string(input_file)
        .expect("Failed to read input file");
    let max_x = input.lines().take(1).map(|line| line.len()).next().unwrap() as i32;
    let max_y = input.lines().count() as i32;
    let map: HashMap<char, Vec<Point>> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line
            .chars()
            .enumerate()
            .filter(|(_, char)| char != &'.')
            .map(move |(x, char)| (Point(x as i32, y as i32), char))
        )
        .fold(HashMap::new(), 
            |mut by_char, (coord, char)| {
                by_char
                    .entry(char)
                    .or_default()
                    .push(coord);
                by_char
            });
    (map, max_x, max_y)
}

fn calc_coord_pairs(coords: &Vec<Point>) -> impl Iterator<Item = (&Point, &Point)> {
    coords
        .iter()
        .enumerate()
        .flat_map(|(index, coord)| coords
            .iter()
            .skip(index + 1)
            .map(move |coord2| (coord, coord2))
        )
}

fn find_antinodes_for_coord_pair(coord1: &Point, coord2: &Point, max_x: i32, max_y: i32) -> Vec<Point> {
    let mut result = vec![*coord1, *coord2];
    let diff = coord1.safe_substract(&coord2).unwrap();
    let mut minimal_point = coord1.safe_add(Some(diff));
    let mut is_valid = minimal_point.is_some_and(|coords| coords.is_within_limits(max_x, max_y));
    while is_valid {
        let unwrapped_minimal = minimal_point.unwrap();
        result.push(unwrapped_minimal);
        minimal_point = unwrapped_minimal.safe_add(Some(diff));
        is_valid = minimal_point.is_some_and(|coords| coords.is_within_limits(max_x, max_y))
    }

    let mut maximal_point = coord1.safe_substract(&diff);
    let mut is_valid = maximal_point.is_some_and(|coords| coords.is_within_limits(max_x, max_y));
    while is_valid {
        let unwrapped_maximal = maximal_point.unwrap();
        result.push(unwrapped_maximal);
        maximal_point = unwrapped_maximal.safe_substract(&diff);
        is_valid = maximal_point.is_some_and(|coords| coords.is_within_limits(max_x, max_y))
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_point_ops() {
        let first = Point(8, 1);
        let second = Point(5, 2);
        assert_eq!(first.safe_substract(&second), Some(Point(3, -1)));
        assert_eq!(first.safe_add(Some(Point(3, -1))), Some(Point(11, 0)));
    }

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1("./input_test/day_08.txt"), 14);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("./input_test/day_08.txt"), 34);
    }
}
