use std::fs;

// Main function
pub fn day_18() {
    let input_file = "./input/day_18.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

#[cfg(test)]
fn get_grid_size() -> usize {
    6
}

#[cfg(not(test))]
fn get_grid_size() -> usize {
    70
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> usize {
    let input: Vec<(usize, usize)> = fs::read_to_string(input_file)
        .expect("Failed to read input file")
        .lines()
        .map(|line| {
            let mut coord_iter = 
                line
                .split(',')
                .map(|num| num
                    .parse()
                    .expect("Expecting number")
                );
            (coord_iter.next().expect("Expecting x coord"), coord_iter.next().expect("Expecting y coord"))
        }).collect();
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
        assert_eq!(puzzle1("./input_test/day_18.txt"), 0);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("./input_test/day_18.txt"), 0);
    }
}
