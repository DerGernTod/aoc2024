use std::{collections::VecDeque, fs, vec};

#[derive(Clone, Debug)]
struct DiskSpace {
    size: usize,
    file_id: Option<usize>,
    start: usize
}

impl DiskSpace {
    fn new(size: usize, file_id: Option<usize>, start: usize) -> DiskSpace {
        DiskSpace {
            size, 
            file_id,
            start
        }
    }
}

// Main function
pub fn day_09() {
    let input_file = "./input/day_09.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> usize {
    let mut entries: VecDeque<Option<usize>> = fs::read_to_string(input_file)
        .expect("Failed to read input file")
        .chars()
        .map(|char| char.to_digit(10).expect("Expecting all chars to be digits") as usize)
        .enumerate()
        .flat_map(|(file_id, count)| {
            if file_id % 2 == 0 {
                vec![Some(file_id / 2); count]
            } else {
                vec![None; count]
            }
        })
        .collect();
    let mut populated = vec![];
    
    while let Some(entry) = entries.pop_front() {
        if let Some(file_id) = entry {
            populated.push(file_id);
        } else {
            // pop until we found `Some`
            while let Some(last_entry) = entries.pop_back() {
                if let Some(last_entry_file_id) = last_entry {
                    populated.push(last_entry_file_id);
                    break;
                }
            }  
        }
    }

    populated
        .iter()
        .enumerate()
        .map(|(index, file_id)| index * file_id)
        .sum()
}

// Puzzle 2 function
fn puzzle2(input_file: &str) -> usize {
    let (_, mut entries, mut empty_spaces) = fs::read_to_string(input_file)
        .expect("Failed to read input file")
        .chars()
        .map(|char| char.to_digit(10).expect("Expecting all chars to be digits") as usize)
        .enumerate()
        .fold((0, vec![], vec![]), |(index, mut entries, mut empty_spaces), (start, size)| {
            let file_id = start / 2;
            if start % 2 == 0 {
                entries.push(DiskSpace::new(size, Some(file_id), index));
            } else {
                empty_spaces.push(DiskSpace::new(size, None, index));
            }
            (index + size, entries, empty_spaces)
        });
    

    for entry in entries.iter_mut().rev() {
        if let Some(empty_space) = empty_spaces
            .iter_mut()
            .find(|space| space.size >= entry.size && space.start < entry.start)
        {
            entry.start = empty_space.start;
            empty_space.size -= entry.size;
            empty_space.start += entry.size;
        }
    }
    entries
        .iter()
        .flat_map(|entry| (0..entry.size)
            .map(|id| (entry.start + id) * entry.file_id.unwrap()))
        .sum()
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
        assert_eq!(puzzle2("./input_test/day_09.txt"), 2858);
    }
}
