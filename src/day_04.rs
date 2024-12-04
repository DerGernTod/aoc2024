use std::{collections::HashMap, fs};

#[allow(dead_code)]
pub fn day_04() {
    let num_xmas = count_xmas("./input/day_04.txt");
    println!("num xmases: {}", num_xmas);
    let num_cross_mas = count_cross_mas("./input/day_04.txt");
    println!("num crossmases: {}", num_cross_mas);
}


fn count_xmas(path: &str) -> usize {
    let map = read_map(path);
    map
        .iter()
        .filter(|(_, char)| char == &&'X')
        .map(|((x, y), _)| get_surrounding_matches(x, y, &map))
        .sum()
}

fn count_cross_mas(path: &str) -> usize {
    let map = read_map(path);
    map
        .iter()
        .filter(|(_, char)| char == &&'A')
        .filter(|((x, y), _)| has_diag_mas(x, y, &map))
        .count()
}

fn has_diag_mas(x: &i32, y: &i32, map: &HashMap<(i32, i32), char>) -> bool {
    let mut res = 0;
    for k in -1..=1 {
        for j in -1..=1 {
            if k == 0 || j == 0 {
                continue;
            }
            if matches_cross_mas_in_direction(x, y, &(k, j), map) {
                res += 1;
            }
        }
    }
    res == 2
}

fn matches_cross_mas_in_direction(x: &i32, y: &i32, dir: &(i32, i32), map: &HashMap<(i32, i32), char>) -> bool {
    if let Some(opposite_char) = map.get(&(x - dir.0, y - dir.1)) {
        if let Some(char_dir) = map.get(&(x + dir.0, y + dir.1)) {
            return match (opposite_char, char_dir) {
                ('M','S') => true,
                _ => false
            }
        }
    }
    false
}

fn read_map(path: &str) -> HashMap<(i32, i32), char> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y , line)|
            line
                .chars()
                .enumerate()
                .map(move |(x, char)| ((x.try_into().unwrap(), y.try_into().unwrap()), char))
        )
        .collect()
}

fn get_surrounding_matches(x: &i32, y: &i32, map: &HashMap<(i32, i32), char>) -> usize {
    let mut res = 0;
    for k in -1..=1 {
        for j in -1..=1 {
            if k == 0 && j == 0 {
                continue;
            }
            if matches_mas_in_direction(x, y, &(k, j), 'M', map) {
                res += 1;
            }
        }
    }
    res
}

fn matches_mas_in_direction(x: &i32, y: &i32, dir: &(i32, i32), needle: char, map: &HashMap<(i32, i32), char>) -> bool {
    let dir_multiplier = match needle {
        'M' => 1,
        'A' => 2,
        'S' => 3,
        _ => panic!("Didn't expect another char!")
    };
    if let Some(char_at_pos) = map.get(&(x + dir.0 * dir_multiplier, y + dir.1 * dir_multiplier)) {
        if char_at_pos == &needle {
            return match needle {
                'M' => matches_mas_in_direction(x, y, &dir, 'A', map),
                'A' => matches_mas_in_direction(x, y, &dir, 'S', map),
                'S' => {
                    true
                },
                _ => panic!("Didn't expect another char!")
            };
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use crate::day_04::*;

    #[test]
    fn test_part_1() {
        assert_eq!(count_xmas("./input_test/day_04.txt"), 18);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(count_cross_mas("./input_test/day_04.txt"), 9);
    }
}