use std::{collections::HashMap, fs, ops::{Add, AddAssign, Sub, SubAssign}};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Vec2D(isize, isize);

impl Add for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vec2D {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Vec2D {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Sub for Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2D(self.0 - rhs.0, self.1 - rhs.1)
    }
}

// Main function
pub fn day_15() {
    let input_file = "./input/day_15.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> isize {
    let (mut map, commands) = read_map_commands(input_file);
    let mut robot_pos = find_start(&map);
    for command in commands {
        let mut potential_target = robot_pos;
        let mut chars_to_insert = vec![];
        let should_move = loop {
            potential_target += command;
            let next_map_entry = map.get(&potential_target);
            
            match next_map_entry {
                Some('.') => {
                    break true;
                },
                Some('O') => {
                    chars_to_insert.push('O');
                },
                Some('#') => {
                    chars_to_insert.clear();
                    break false;   
                },
                _ => panic!("Expected valid character at {:?} but got {:?}", potential_target, next_map_entry)
            }
        };
        
        if should_move {
            while let Some(char) = chars_to_insert.pop() {
                map.entry(potential_target).and_modify(|map_pos| *map_pos = char);
                potential_target -= command
            }
            map.entry(robot_pos).and_modify(|map_pos| *map_pos = '.');
            robot_pos += command;
            map.entry(robot_pos).and_modify(|map_pos| *map_pos = '@');
        }
        print_map(&map);
    }
    calc_gps_pos(&map)
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

    let map_str = inputs.swap_remove(0);
    let map: HashMap<Vec2D, char> = read_map(map_str);
    let command_str = inputs.swap_remove(0);
    let commands: Vec<Vec2D> = read_commands(command_str);
        
    (map, commands)

}

fn read_map(map_str: &str) -> HashMap<Vec2D, char> {
    map_str
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| line
            .chars()
            .enumerate()
            .map(move |(x, char)| (Vec2D(x as isize, y as isize), char)))
        .collect()    
}

fn read_commands(command_str: &str) -> Vec<Vec2D> {
    command_str
        .lines()
        .flat_map(|line| line.chars())
        .map(|char| match char {
            '<' => Vec2D(-1, 0),
            'v' => Vec2D(0, 1),
            '>' => Vec2D(1, 0),
            '^' => Vec2D(0, -1),
            _ => panic!("Unexpected command!")
        })
        .collect()
}

fn read_map_wide_commands(input_file: &str) -> (HashMap<Vec2D, char>, Vec<Vec2D>) {
    let inputs = fs::read_to_string(input_file)
        .expect("Failed to read input file");
    let mut inputs: Vec<&str> = inputs
        .split("\n\n")
        .collect();

    let map_str = inputs.swap_remove(0);
    let map: HashMap<Vec2D, char> = read_map_wide(map_str);
    let command_str = inputs.swap_remove(0);
    let commands: Vec<Vec2D> = read_commands(command_str);
        
    (map, commands)
}
fn read_map_wide(map_str: &str) -> HashMap<Vec2D, char> {
    map_str
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| line
            .chars()
            .enumerate()
            .flat_map(move |(x, char)| {
                let first_loc = Vec2D(x as isize * 2, y as isize);
                let second_loc = Vec2D(x as isize * 2 + 1, y as isize);
                match char {
                    '@' => [(first_loc, '@'), (second_loc, '.')],
                    'O' => [(first_loc, '['), (second_loc, ']')],
                    x => [(first_loc, char), (second_loc, char)]
                }
            })
        )
        .collect()    
}

fn print_map(map: &HashMap<Vec2D, char>) {
    for y in 0..20 {
        for x in 0..20 {
            if let Some(ch) = map.get(&Vec2D(x, y)) {
                print!("{}", ch);
            } else {
                print!(".");
            }
        }
        println!();

    }
    println!();
}

fn calc_gps_pos_with(map: &HashMap<Vec2D, char>, find_char: char, divisor: isize) -> isize {
    map
        .iter()
        .filter(|(_, &char)| char == find_char)
        .map(|(coords, _)| coords.0 / divisor + coords.1 * 100)
        .sum()
}

fn calc_gps_pos(map: &HashMap<Vec2D, char>) -> isize {
    calc_gps_pos_with(map, 'O', 1)
}

fn calc_gps_pos_wide(map: &HashMap<Vec2D, char>) -> isize {
    calc_gps_pos_with(map, '[', 2)
}

// Puzzle 2 function
fn puzzle2(input_file: &str) -> usize {
    let (map, commands) = read_map_wide_commands(input_file);
    print_map(&map);
    let start = find_start(&map);

    for command in commands {
        if can_execute_command(&map, &start, &command) {

        }
    }

    println!("Start: {:?}", start);
    0
}

fn can_execute_command(map: &HashMap<Vec2D, char>, start_pos: Vec2D, command: Vec2D) -> bool {
    let cur_char = map.get(&start_pos).expect(&format!("Expecting valid position at {:?}", start_pos));
    match cur_char {
        '#' => false,
        '.' => true,
        x => panic!("Unexpected character found while checking command: {}!", x)
    }
}

fn execute_command(map: &mut HashMap<Vec2D, char>, start_pos: Vec2D, command: Vec2D) {
    let cur_char = map.get(&start_pos).expect(&format!("Expecting valid position at {:?}", start_pos));
    match cur_char {
        '@' => execute_command(map, start_pos + command, command),
        '.' => (),
        '[' => {
            execute_command(map, start_pos + command, command);
            execute_command(map, start_pos + Vec2D(1, 0), command);
        }
        x => panic!("Unexpected character found while executing command: {}!", x)
    }
    
    let prev_pos = start_pos - command;
    let prev_char = map.get_mut(&prev_pos).expect(&format!("Expecting valid position at {:?}", prev_pos));
    let cur_char = map.get_mut(&start_pos).expect(&format!("Expecting valid position at {:?}", start_pos));
    *cur_char = *prev_char;
    *prev_char = '.';
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
