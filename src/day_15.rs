use std::{collections::HashMap, fs};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2D(isize, isize);

impl Add for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}

// Main function
pub fn day_15() {
    let input_file = "./input/day_15.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> usize {
    let (mut map, commands) = read_map_commands(input_file);
    let start = find_start(&map);
    for command in commands {
        let potential_target = start + command;

        let map_content = map.entry(potential_target);
    }
    10092
}

fn find_start(map: &HashMap<Vec2D, char>) -> Vec2D {
    *map
        .iter()
        .find(|(_, &char)| char == '@')
        .expect("Expecting start position to be in map!")
        .0
}

fn read_map_commands(input_file: &str) -> (HashMap<Vec2D, char>, Vec<Vec2D>) {
    let inputs = fs::read_to_string(input_file)
        .expect("Failed to read input file");
    let mut inputs: Vec<&str> = inputs
        .split("\n\n")
        .collect();

    let map: HashMap<Vec2D, char> = inputs
        .swap_remove(0)
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| line
            .chars()
            .enumerate()
            .map(move |(x, char)| (Vec2D(x as isize, y as isize), char)))
        .collect();

    let commands: Vec<Vec2D> = inputs
        .swap_remove(0)
        .lines()
        .flat_map(|line| line.chars())
        .map(|char| match char {
            '<' => Vec2D(-1, 0),
            'v' => Vec2D(0, 1),
            '>' => Vec2D(1, 0),
            '^' => Vec2D(0, -1),
            _ => panic!("Unexpected command!")
        }).collect();
    (map, commands)

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
        assert_eq!(puzzle1("./input_test/day_15.txt"), 10092);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("./input_test/day_15.txt"), 0);
    }
}
