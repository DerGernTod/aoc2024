use std::{cmp::Ordering, collections::{HashMap, HashSet}, fs, usize};
#[allow(dead_code)]
pub fn day_05() {
    let res = calc_ordered_middle_sum("./input/day_05.txt");
    println!("result: {}", res);
    let res = calc_unordered_middle_sum("./input/day_05.txt");
    println!("result2: {}", res);
}

fn read_input(path: &str) -> (String, HashMap<usize, HashSet<usize>>, HashMap<usize, HashSet<usize>>) {
    let str = fs::read_to_string(path).unwrap();
    let mut splits = str.split("\n\n");
    let sorts = splits.next().unwrap();
    let updates = splits.next().unwrap();
    let mut lookup: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut reverse_lookup: HashMap<usize, HashSet<usize>> = HashMap::new();
    (lookup, reverse_lookup) = sorts
        .split("\n")
        .map(|sort| {
            let mut order_iter = sort
                .split("|")
                .map(|spl| spl.parse::<usize>().unwrap());
            (order_iter.next().unwrap(), order_iter.next().unwrap())
        })
        .fold((lookup, reverse_lookup), | (mut cur_lookup, mut cur_reverse_lookup), (before, after) | {
            let entry = cur_lookup
                .entry(before)
                .or_insert(HashSet::new());
            entry.insert(after);
            let reverse_entry = cur_reverse_lookup
                .entry(after)
                .or_insert(HashSet::new());
            reverse_entry.insert(before);
            (cur_lookup, cur_reverse_lookup)
        });
    (String::from(updates), lookup, reverse_lookup)
}

struct Sorter {
    lookup: HashMap<usize, HashSet<usize>>,
    reverse_lookup: HashMap<usize, HashSet<usize>>
}

impl Sorter {
    pub fn new(lookup: HashMap<usize, HashSet<usize>>, reverse_lookup: HashMap<usize, HashSet<usize>>) -> Sorter {
        Sorter {
            lookup,
            reverse_lookup
        }
    }
    pub fn all_sorted(&self, window: &[usize]) -> bool {
        return self.sort(window[0], window[1]) == Ordering::Less;
    }
    pub fn sort(&self, a: usize, b: usize) -> Ordering {
        let is_b_correctly_after_a = self.lookup
            .get(&a)
            .map(|before_all| before_all.get(&b).is_some());
        let is_a_correctly_before_b = self.reverse_lookup
            .get(&b)
            .map(|after_all| after_all.get(&a).is_some());
        
        match (is_b_correctly_after_a, is_a_correctly_before_b) {
            (Some(true), Some(true)) => Ordering::Less,
            (Some(a), _) => if a { Ordering::Less } else { Ordering::Equal },
            (_, Some(b)) => if b { Ordering::Less } else { Ordering::Equal },
            (_, _) => Ordering::Equal
        }
    }
}

fn calc_unordered_middle_sum(path: &str) -> usize {
    let (updates, lookup, reverse_lookup) = read_input(path);
    let sorter = Sorter::new(lookup, reverse_lookup);
    updates
        .split("\n")
        .map(|update| update
            .split(",")
            .map(|page| page.parse::<usize>().unwrap())
            .collect::<Vec<usize>>())
        .filter(|update| {
            !update.windows(2).all(|window| sorter.all_sorted(window))
        })
        .inspect(|update| println!("update is unordered: {:?}", update))
        .map(|update| {
            let mut cloned = update.clone();
            cloned.sort_by(|a, b| sorter.sort(*a, *b));
            cloned
        })
        .map(|update| *update.get(update.len() / 2).unwrap())
        .sum()

}

fn calc_ordered_middle_sum(path: &str) -> usize {
    let (updates, lookup, reverse_lookup) = read_input(path);
    let sorter = Sorter::new(lookup, reverse_lookup);
    updates
    .split("\n")
    .map(|update| update
        .split(",")
        .map(|page| page.parse::<usize>().unwrap())
        .collect::<Vec<usize>>())
    .filter(|update| update.windows(2).all(|window| sorter.all_sorted(window)))
    .inspect(|update| println!("update is ordered: {:?}", update))
    .map(|update| *update.get(update.len() / 2).unwrap())
    .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_05::*;

    #[test]
    fn test_part_1() {
        assert_eq!(calc_ordered_middle_sum("./input_test/day_05.txt"), 143);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(calc_unordered_middle_sum("./input_test/day_05.txt"), 123);
    }
}