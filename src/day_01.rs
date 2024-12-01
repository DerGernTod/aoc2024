use std::{borrow::BorrowMut, collections::{btree_map::Values, HashMap}, fs, ptr::read};


trait NumCounter {
    fn count_occurrences(&self) -> HashMap<usize, usize>;
}

impl NumCounter for Vec<usize> {
    fn count_occurrences(&self) -> HashMap<usize, usize> {
        let mut result_map = HashMap::new();
        for val in self {
            let entry = result_map.entry(*val).or_insert(0);
            *entry += 1;
        }
        result_map
    }
}

struct Lists {
    left: Vec<usize>,
    right: Vec<usize>
}



impl Lists {
    pub fn new() -> Lists {
        Lists {
            left: vec![],
            right: vec![]
        }
    }
    pub fn add<T>(&mut self, mut iter: T) -> &mut Lists where T: Iterator<Item = usize> {
        self.left.push(iter.next().unwrap());
        self.right.push(iter.next().unwrap());
        self
    }
    pub fn sort(&mut self) {
        self.left.sort();
        self.right.sort();
    }
    pub fn calc_similarity(&self) -> usize {
        let left_map = self.left.count_occurrences();
        let right_map = self.right.count_occurrences();
        let mut count = 0;
        for (key, value) in left_map {
            count += key * right_map.get(&key).unwrap_or(&0) * value;
        }
        count
    }
    pub fn calc_pair_diff(&self) -> usize {
        self.left.iter()
            .zip(self.right.iter())
            .map(|(left, right)| left.abs_diff(*right))
            .sum()
    }
}

#[allow(dead_code)]
pub fn day_01() {
    let distance = calc_distances("./input/day_01.txt");
    println!("distance is {}", distance);

    let similarity = calc_similarity("./input/day_01.txt");
    println!("similarity is {}", similarity);
}

fn read_to_lists(path: &str) -> Lists {
    let mut result_list = Lists::new();
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line: &str| line
            .split_whitespace()
            .map(|str| str.parse::<usize>().unwrap())
        )
        .fold(result_list.borrow_mut(), |init, iter| init.add(iter));
    result_list
}

fn calc_distances(path: &str) -> usize {
    let mut result_list = read_to_lists(path);
    result_list.sort();

    result_list.calc_pair_diff()
}

fn calc_similarity(path: &str) -> usize {
    let result_list = read_to_lists(path);
    result_list.calc_similarity()
}

#[cfg(test)]
mod tests {
    use crate::day_01::*;

    #[test]
    fn test_part_1() {
        let sum = calc_distances("./input_test/day_01.txt");
        assert_eq!(sum, 11);
    }

    #[test]
    fn test_part_2() {
        let similarity = calc_similarity("./input_test/day_01.txt");
        assert_eq!(similarity, 31);
    }
}