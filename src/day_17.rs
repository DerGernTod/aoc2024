use std::{collections::{HashSet, VecDeque}, fs, iter::Rev, thread::sleep, time::Duration};
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Registry {
    a: u64,
    b: u64,
    c: u64
}

fn execute(registry: &mut Registry, exec_index: u64, operator: u64, operand: u64, out_val: &mut Vec<u64>, lookup: &mut HashSet<(Registry, u64)>) -> Option<u64> {
    let combo_value = evaluate_operand(registry, operand);
    match operator {
        0 => {
            registry.a = registry.a >> combo_value;
            Some(exec_index + 2)
        },
        1 => {
            registry.b ^= operand;
            Some(exec_index + 2)
        },
        2 => {
            registry.b = combo_value % 8;
            Some(exec_index + 2)
        },
        3 => {
            if registry.a == 0 {
                Some(exec_index + 2)
            } else if lookup.contains(&(*registry, operand)) {
                None
            } else {
                lookup.insert((*registry, operand));
                Some(operand)
            }
        },
        4 => {
            registry.b ^= registry.c;
            Some(exec_index + 2)
        },
        5 => {
            out_val.push(combo_value % 8);
            Some(exec_index + 2)
        },
        6 => {
            registry.b = registry.a >> combo_value;
            Some(exec_index + 2)
        },
        7 => {
            registry.c = registry.a >> combo_value;
            Some(exec_index + 2)
        },
        _ => panic!("Invalid operand: {}", operator)
    }
}

fn evaluate_operand(registry: &Registry, val: u64) -> u64 {
    match val {
        0..=3 => val,
        4 => registry.a,
        5 => registry.b,
        6 => registry.c,
        _ => panic!("Invalid operand")
    }
}


// Main function
pub fn day_17() {
    let input_file = "./input/day_17.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> String {
    let (registry, ops) = read_registry_and_ops(input_file);
    
    let out_vals = execute_operations(registry, &ops, false, registry.a).expect("Expecting output values for puzzle 1");
    out_vals
        .iter()
        .map(|val| val.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn execute_operations(mut registry: Registry, ops: &Vec<u64>, check_equality: bool, init_val: u64) -> Option<Vec<u64>> {
    let mut pointer = 0;
    let mut out_vals: Vec<u64> = Vec::new();
    let mut lookup: HashSet<(Registry, u64)> = HashSet::new();
    while pointer < ops.len() {
        let operator = ops.get(pointer).expect("Expecting operator");
        let operand = ops.get(pointer + 1).expect("Expecting operand");
        pointer = execute(&mut registry, pointer as u64, *operator, *operand, &mut out_vals, &mut lookup)? as usize;
        if check_equality && !vec_starts_with(ops, &out_vals) {
            return None;
        // } else if *operator == 5 && out_vals.len() > 9 {
        //     println!("Output for a={:o}: {:?}", init_val, out_vals);
        //     std::thread::yield_now();
        }
    }
    Some(out_vals)
}

fn vec_starts_with(ops: &Vec<u64>, out_vals: &Vec<u64>) -> bool {
    for i in 0..out_vals.len() {
        if let Some(op) = ops.get(i) {
            if *op != out_vals[i] {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn read_registry_and_ops(input_file: &str) -> (Registry, Vec<u64>) {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    let mut registries = input
        .lines()
        .take(3)
        .flat_map(|line| line
            .split_whitespace()
            .skip(2)
            .map(|str| str.parse().expect("Expecting u64 for registry entry")));
    let registry = Registry {
        a: registries.next().expect("Expecting registry entry A"),
        b: registries.next().expect("Expecting registry entry B"),
        c: registries.next().expect("Expecting registry entry C")
    };
    let program = input
        .lines()
        .skip(4)
        .next()
        .expect("Expecting program line")
        .split_whitespace()
        .skip(1)
        .flat_map(|str| str
            .split(',')
            .flat_map(|num| num
                .parse()
                .map_err(|_| "Expecting u64 for program entry")))
        .collect();
    (registry, program)
}

// Puzzle 2 function
fn puzzle2(input_file: &str) -> u64 {
    let (registry, ops) = read_registry_and_ops(input_file);
    let mut a = 0o6032004233;
    loop {
        let registry = Registry {
            a,
            b: 0,
            c: 0
        };
        if let Some(out_vals) = execute_operations(registry, &ops, true, a) {
            if out_vals == ops {
                return a;
            }
        }
        a += 0o10000000000;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1("./input_test/day_17.txt"), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("./input/day_17.txt"), 107416870455451);
    }
}
