use std::{collections::{HashMap, HashSet}, fs, ops::Add};

// Main function
pub fn day_06() {
    let input_file = "./input/day_06.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    let (start_pos, map) = calc_map(input);
    let result = walk(start_pos, &map, HashSet::new(), (0, -1));
    result.len()
}

fn calc_map(input: String) -> ((usize, usize), HashMap<(usize, usize), char>) {
    let mut start_pos = (0,0);
    let mut map = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '^' {
                start_pos = (x, y);
            }
            map.insert((x, y), char);
        }
    }
    (start_pos, map)
}

fn walk(
    start_pos: (usize, usize),
    map: &HashMap<(usize, usize), char>,
    mut marks: HashSet<(usize, usize)>,
    dir: (i32, i32)
) -> HashSet<(usize, usize)> {
    marks.insert(start_pos);

    let x = (start_pos.0 as i32) + dir.0;
    let y = (start_pos.1 as i32) + dir.1;
    if x < 0 || y < 0 {
        return marks;
    }
    let new_pos = (x as usize, y as usize);
    
    match map.get(&new_pos) {
        Some('#') => {
            walk(start_pos, map, marks, rotate(dir))
        },
        Some(_) => {
            walk(new_pos, map, marks, dir)
        },
        None => marks
    }
}

fn rotate(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (0, 1) => (-1, 0),
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (-1, 0) => (0, -1),
        _ => panic!("Invalid direction rotation: {:?}", dir)
    }
}

// Puzzle 2 function
fn puzzle2(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    let (start_pos, map) = calc_map(input);
    let mut options = walk_dir(start_pos, &map, HashSet::new(), (0, -1), HashSet::new());
    options.remove(&start_pos);
    options.len()
}

fn walk_dir(
    start_pos: (usize, usize),
    map: &HashMap<(usize, usize), char>,
    mut marks: HashSet<((usize, usize), (i32, i32))>,
    dir: (i32, i32),
    mut options: HashSet<(usize, usize)>
) -> HashSet<(usize, usize)> {
    marks.insert((start_pos, dir));

    let x = (start_pos.0 as i32) + dir.0;
    let y = (start_pos.1 as i32) + dir.1;
    if x < 0 || y < 0 {
        return options;
    }
    let new_pos = (x as usize, y as usize);
    
    match map.get(&new_pos) {
        Some('#') => {
            walk_dir(start_pos, map, marks, rotate(dir), options)
        },
        Some(_) => {
            // try walk right
            if !options.contains(&new_pos)
                && !contains_any_dir(new_pos, &marks)
                && returns_to_path(start_pos, map, &marks, rotate(dir), HashSet::new(), new_pos) {
                options.insert(new_pos);
            }
            walk_dir(new_pos, map, marks, dir, options)
        },
        None => options
    }
}

fn contains_any_dir(check_pos: (usize, usize), marks: &HashSet<((usize, usize), (i32, i32))>) -> bool {
    let checks = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for dir in checks {
        if marks.contains(&(check_pos, dir)) {
            return true;
        }
    }
    false
}

fn returns_to_path(
    start_pos: (usize, usize),
    map: &HashMap<(usize, usize), char>,
    marks: &HashSet<((usize, usize), (i32, i32))>,
    dir: (i32, i32),
    mut visited_check: HashSet<((usize, usize), (i32, i32))>,
    option: (usize, usize)
) -> bool {
    let start_entry = (start_pos, dir);
    if visited_check.contains(&start_entry) || marks.contains(&start_entry) {
        return true;
    }
    visited_check.insert(start_entry);
    let x = (start_pos.0 as i32) + dir.0;
    let y = (start_pos.1 as i32) + dir.1;
    if x < 0 || y < 0 {
        return false;
    }
    let new_pos = (x as usize, y as usize);
    if new_pos == option {
        return returns_to_path(start_pos, map, marks, rotate(dir), visited_check, option);
    }
    match map.get(&new_pos) {
        Some('#') => {
            returns_to_path(start_pos, map, marks, rotate(dir), visited_check, option)
        },
        Some(_) => {
            returns_to_path(new_pos, map, marks, dir, visited_check, option)
        },
        None => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        // Replace with proper test logic
        assert_eq!(puzzle1("./input_test/day_06.txt"), 41);
    }

    #[test]
    fn test_puzzle2() {
        // Replace with proper test logic
        assert_eq!(puzzle2("./input_test/day_06.txt"), 6);
    }
}
