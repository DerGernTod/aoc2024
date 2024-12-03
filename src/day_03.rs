use std::fs;
use regex::Regex;

#[allow(dead_code)]
pub fn day_03() {
    let sum = sum_mul_commands("./input/day_03.txt");
    println!("sum mul commands: {}", sum);
    let do_mul_sum = sum_enabled_mul_commands("./input/day_03.txt");
    println!("sum do mul commands: {}", do_mul_sum);
}

fn sum_enabled_mul_commands(path: &str) -> usize {
    let input = fs::read_to_string(path).unwrap();
    let dont_splits = input.split("don't()");
    let mut sum = 0;
    for (i, spl) in dont_splits.enumerate() {
        if i == 0 {
            sum += sum_mul_str(spl);
            continue;
        }
        let str_iter = spl.split_once("do()");
        
        if let Some((_, do_str)) = str_iter {
            sum += sum_mul_str(&do_str);
        }
    }
    sum
}

fn sum_mul_commands(path: &str) -> usize {
    let input = fs::read_to_string(path).unwrap();
    sum_mul_str(&input)
}

fn sum_mul_str(input: &str) -> usize {
    let find_muls_pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    find_muls_pattern
        .captures_iter(input)
        .filter_map(|captures|
            match (captures.get(1), captures.get(2)) {
                (Some(a), Some(b)) => Some(
                    a.as_str().parse::<usize>().unwrap()
                    * b.as_str().parse::<usize>().unwrap()
                ),
                _ => None,
            })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_03::*;

    #[test]
    fn test_part_1() {
        assert_eq!(sum_mul_commands("./input_test/day_03.txt"), 161);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(sum_enabled_mul_commands("./input_test/day_03_2.txt"), 48);
    }
}