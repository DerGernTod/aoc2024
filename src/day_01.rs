use std::{borrow::BorrowMut, fs};

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
}


fn calc_distances(path: &str) -> usize {
    let mut result_list = Lists::new();
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line: &str| line
            .split_whitespace()
            .map(|str| str.parse::<usize>().unwrap())
        )
        .fold(result_list.borrow_mut(), |init, iter| init.add(iter));
    result_list.sort();

    result_list.calc_pair_diff()
}

#[cfg(test)]
mod tests {
    use crate::day_01::*;

    #[test]
    fn test_part_1() {
        let sum = calc_distances("./input_test/day_01.txt");
        assert_eq!(sum, 11);
    }
}