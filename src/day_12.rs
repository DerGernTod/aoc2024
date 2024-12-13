use std::{collections::{HashMap, HashSet, VecDeque}, fs, ops::Neg};

#[derive(PartialEq, PartialOrd, Eq, Hash, Copy, Clone, Debug)]
struct Point(i32, i32);

impl Point {
    fn get_surroundings(&self) -> Vec<Option<Point>> {
        let (right, bot, left, top) = self.get_surroundings_clockwise();
        vec![
            right,
            bot,
            left,
            top,
        ]
    }

    fn get_surroundings_clockwise(&self) -> (Option<Point>, Option<Point>, Option<Point>, Option<Point>) {
        (
            self.checked_add(&Point(1, 0)),
            self.checked_sub(&Point(0, 1)),
            self.checked_sub(&Point(1, 0)),
            self.checked_add(&Point(0, 1)),
        )
    }

    fn checked_add(&self, rhs: &Point) -> Option<Point> {
        let x = self.0.checked_add(rhs.0)?;
        let y = self.1.checked_add(rhs.1)?;
        Some(Point(x, y))
    }

    fn checked_sub(&self, rhs: &Point) -> Option<Point> {
        let x = self.0.checked_sub(rhs.0)?;
        let y = self.1.checked_sub(rhs.1)?;
        Some(Point(x, y))
    }
}

// Main function
pub fn day_12() {
    let input_file = "./input/day_12.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> i32 {
    read_to_map(input_file)
        .into_iter()
        .flat_map(|(_, mut points)| {
            find_connected_regions(&mut points)
            .into_iter()
            .map(move |region| region.len() as i32 * count_region_borders(&region))
        })
        .sum()
}

fn count_region_borders(points: &HashSet<Point>) -> i32 {
    points
        .iter()
        .flat_map(|point| point.get_surroundings())
        .filter(|point| !point.is_some_and(|point| points.contains(&point)))
        .count() as i32
}

fn find_connected_regions(points: &mut HashSet<Point>) -> Vec<HashSet<Point>> {
    if points.len() == 0 {
        return vec![];
    }
    let mut totals = vec![];

    while points.len() > 0 {
        totals.push(find_next_connected_region(points));
    }
    totals
}

fn find_next_connected_region(points: &mut HashSet<Point>) -> HashSet<Point> {
    let mut result = HashSet::new();
    let first = *points.iter().next().unwrap();
    points.remove(&first);
    let mut queue = VecDeque::new();
    queue.push_back(first);
    result.insert(first);
    while let Some(entry) = queue.pop_front() {
        (queue, result) = entry
            .get_surroundings()
            .into_iter()
            .flatten()
            .flat_map(|point| points.remove(&point).then_some(point))
            .fold((queue, result), |(mut queue, mut result), key_val| {
                queue.push_back(key_val);
                result.insert(key_val);
                (queue, result)
            });
    }
    result
}

fn read_to_map(input_file: &str) -> HashMap<char, HashSet<Point>> {
    fs::read_to_string(input_file)
        .expect("Failed to read input file")
        .lines()
        .enumerate()
        .fold(HashMap::new(), |map, (y, line)| {
            line
                .chars()
                .enumerate()
                .fold(map, |mut map, (x, char)| {
                    let entry = map.entry(char).or_default();
                    entry.insert(Point(x as i32, y as i32));
                    map
            })
        })
}

// Puzzle 2 function
fn puzzle2(input_file: &str) -> i32 {
    read_to_map(input_file)
        .into_iter()
        .flat_map(|(_, mut points)| {
            find_connected_regions(&mut points)
                .into_iter()
                .map(move |region| region.len() as i32 * count_region_sides(&region))
        })
        .sum()
}

fn count_region_sides(points: &HashSet<Point>) -> i32 {
    let mut border_points = find_border_points(points);
    let mut lines_result = vec![];
    while border_points.len() > 0 {
        check_line(&mut border_points, &mut lines_result);
    }
    lines_result.len() as i32
}

fn find_border_points(points: &HashSet<Point>) -> HashSet<(Point, Point)> {
    points
        .iter()
        .flat_map(|point| point
            .get_surroundings()
            .into_iter()
            .map(|surr_point| surr_point
                .filter(|surr_point| !points.contains(surr_point))
                .map(|surr_point| (*point, surr_point))
            )
        )
        .flatten()
        .collect()
}

fn check_line(outer_points: &mut HashSet<(Point, Point)>, lines: &mut Vec<(Point, Point)>) {
    let start = *outer_points
        .iter()
        .next()
        .unwrap();
    outer_points.remove(&start);
    let up = Point(0, -1);
    let down = Point(0, 1);
    let left = Point(-1, 0);
    let right = Point(1, 0);

    let vert_line = 
        match (find_end_point(outer_points, start, up), find_end_point(outer_points, start, down)) {
            (None, None) => None,
            (Some(start), Some(end)) => Some((start, end)),
            (None, Some(end))
            | (Some(end), None) => Some((start.0, end)),
        };

    let hori_line =
        match (find_end_point(outer_points, start, left), find_end_point(outer_points, start, right)) {
            (None, None) => None,
            (Some(start), Some(end)) => Some((start, end)),
            (None, Some(end))
            | (Some(end), None) => Some((start.0, end))
        };

    if hori_line.is_none() && vert_line.is_none() {
        lines.push((start.0, start.0));
    } else {
        hori_line.map(|line| lines.push(line));
        vert_line.map(|line| lines.push(line));
    }
    
}

fn find_end_point(points: &mut HashSet<(Point, Point)>, next_pos: (Point, Point), dir: Point) -> Option<Point> {
    let mut end = None;
    let mut next_pos = (next_pos.0.checked_add(&dir).unwrap(), next_pos.1.checked_add(&dir).unwrap());
    while let Some(&target) = points.get(&next_pos) {
        points.remove(&target);
        end = Some(target.0);
        next_pos = (next_pos.0.checked_add(&dir).unwrap(), next_pos.1.checked_add(&dir).unwrap());
    }
    end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_region_border() {
        let set = HashSet::from([Point(1, 1), Point(1, 2), Point(0, 1), Point(0, 2)]);
        assert_eq!(count_region_borders(&set), 8)
    }
    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1("./input_test/day_12.txt"), 1930);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("./input_test/day_12.txt"), 368);
    }
}
