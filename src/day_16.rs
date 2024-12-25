use std::{cmp::Ordering, collections::{BinaryHeap, HashMap, HashSet}, fs, ops::{Add, AddAssign, Sub, SubAssign}};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (Vec2D, Vec2D),
}

// Implement ordering for the priority queue
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse order for min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Vec2D(isize, isize);

const RIGHT: usize = 1;
const DIRECTIONS: [Vec2D; 4] = [
    Vec2D(0, -1),
    Vec2D(1, 0),
    Vec2D(0, 1),
    Vec2D(-1, 0),
];

impl Add for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vec2D {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Vec2D {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Sub for Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2D(self.0 - rhs.0, self.1 - rhs.1)
    }
}

// Main function
pub fn day_16() {
    let input_file = "./input/day_16.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> usize {
    let input = read_map(input_file) ;
    let (start, end) = input.iter().fold((None, None), |(start, end), (coords, char)| {
        match char {
            'S' => (Some(*coords), end),
            'E' => (start, Some(*coords)),
            _ => (start, end),
        }
    });
    let start = start.expect("No start found");
    let end = end.expect("No end found");

    let mut costs: HashMap<(Vec2D, Vec2D), usize> = input
        .keys()
        .cloned()
        .flat_map(|coords| DIRECTIONS.into_iter().map(move |dir| (coords, dir)))
        .map(|coords| (coords, usize::MAX))
        .collect();
    costs.get_mut(&(start, DIRECTIONS[RIGHT])).map(|cost| *cost = 0); 
    update_costs(&mut costs, start, end);
    DIRECTIONS.iter().flat_map(|dir| costs.get(&(end, *dir))).min().map(|x| *x).expect("Expecting a value")
}

fn update_costs(costs: &mut HashMap<(Vec2D, Vec2D), usize>, start: Vec2D, end: Vec2D) {
    let mut heap = BinaryHeap::new();
    heap.push(State { cost: 0, position: (start, DIRECTIONS[RIGHT])});

    while let Some(smallest) = heap.pop() {
        let State{ position, cost } = smallest;
        if cost == usize::MAX {
            break;
        }
        if position.0 == end {
            break;
        }
        visit(smallest, costs)
            .into_iter()
            .for_each(|state| heap.push(state));
    }
}

// Puzzle 2 function
fn puzzle2(input_file: &str) -> usize {
    let input = read_map(input_file) ;
    let (start, end) = input.iter().fold((None, None), |(start, end), (coords, char)| {
        match char {
            'S' => (Some(*coords), end),
            'E' => (start, Some(*coords)),
            _ => (start, end),
        }
    });
    let start = start.expect("No start found");
    let end = end.expect("No end found");

    let mut costs: HashMap<(Vec2D, Vec2D), usize> = input
        .keys()
        .cloned()
        .flat_map(|coords| DIRECTIONS.into_iter().map(move |dir| (coords, dir)))
        .map(|coords| (coords, usize::MAX))
        .collect();
    costs.get_mut(&(start, DIRECTIONS[RIGHT])).map(|cost| *cost = 0); 
    update_costs(&mut costs, start, end);

    let costs: HashMap<(Vec2D, Vec2D), usize> = costs
        .into_iter()
        .filter(|&(_, cost)| cost != usize::MAX)
        .collect();
    
    let cur_state = DIRECTIONS
        .into_iter()
        .filter_map(|dir| costs.get_key_value(&(end, dir)))
        .min_by(|(_, cost), (_, cost_b)| cost.cmp(cost_b))
        .map(|res| (*res.0, *res.1))
        .expect("Expeting target state");

    let path_points = check_path_to(&start, cur_state, &costs);
    path_points.len() + 1 // Add the start point
}

fn check_path_to(goal: &Vec2D, cur_state: ((Vec2D, Vec2D), usize), costs: &HashMap<(Vec2D, Vec2D), usize>) -> HashSet<Vec2D> {
    if goal == &cur_state.0.0 {
        return HashSet::new();
    }
    let ((coord, facing), cost) = cur_state;

    let neighbors: Vec<(Vec2D, Vec2D)> = DIRECTIONS
        .iter()
        .filter(|&&dir| dir != facing)
        .map(|&dir| (coord, dir))
        .collect();
    let mut path_points: HashSet<Vec2D> = neighbors
        .into_iter()
        .filter_map(|pos| costs
            .get(&pos)
            .filter(|&&pos_cost| pos_cost + 1000 == cost)
            .map(|cost| (pos, *cost))
        )
        .flat_map(|state| check_path_to(goal, state, costs))
        .collect();

    let prev_state = (coord - facing, facing);
    let prev_cost = costs.get(&prev_state).filter(|&&prev_cost| prev_cost + 1 == cost);
    if let Some(prev_cost) = prev_cost {
        path_points.extend(check_path_to(goal, (prev_state, *prev_cost), costs));
    }

    path_points.insert(cur_state.0.0);
    path_points
}

fn visit(state: State, costs: &mut HashMap<(Vec2D, Vec2D), usize>) -> Vec<State> {
    let mut res = vec![];
    let State { position: (coord, facing), cost } = state;
    let forward = coord + facing;
    costs.entry((forward, facing)).and_modify(|prev_cost| {
        let new_cost = cost + 1;
        if new_cost < *prev_cost {
            *prev_cost = new_cost;
            res.push(State { cost: new_cost, position: (forward, facing) });
        }
    });

    let neighbors: Vec<(Vec2D, Vec2D)> = DIRECTIONS.iter().filter(|&&dir| dir != facing).map(|&dir| (coord, dir)).collect();
    for neighbor in neighbors {
        costs.entry(neighbor).and_modify(|prev_cost| {
            let new_cost = cost + 1000;
            if new_cost < *prev_cost {
                *prev_cost = new_cost;
                res.push(State { cost: new_cost, position: neighbor });
            }
        });
    }
    res
}

fn read_map(input_file: &str) -> HashMap<Vec2D, char> {
    fs::read_to_string(input_file)
        .expect("Failed to read input file")
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| line
            .chars()
            .enumerate()
            .map(move |(x, c)| (Vec2D(x as isize, y as isize), c)))
        .filter(|&(_, char)| char != '#')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1("./input_test/day_16.txt"), 7036);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("./input_test/day_16.txt"), 45);
    }

    #[test]
    fn test_puzzle2_2() {
        assert_eq!(puzzle2("./input_test/day_16_2.txt"), 64);
    }
}
