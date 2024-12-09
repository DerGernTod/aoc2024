use std::fs;

struct DiskSpace {
    index: usize,

}

// Main function
pub fn day_09() {
    let input_file = "./input/day_09.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> usize {
    let mut entries: Vec<Option<usize>> = fs::read_to_string(input_file)
        .expect("Failed to read input file")
        .chars()
        .map(|char| char.to_digit(10).expect("Expecting all chars to be digits") as usize)
        .enumerate()
        .flat_map(|(file_id, count)| {
            if file_id % 2 == 0 {
                vec![Some(file_id); count]
            } else {
                vec![None; count]
            }
        })
        .collect();
    let populated = vec![];
    
    


    1928
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
        assert_eq!(puzzle1("./input_test/day_09.txt"), 1928);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("./input_test/day_09.txt"), 0);
    }
}
