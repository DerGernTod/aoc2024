use std::fs;


#[allow(dead_code)]
pub fn day_02() {
    let safe_reports = count_safe_reports("./input/day_02.txt");
    println!("safe reports: {}", safe_reports);
    let safe_reports = count_safe_reports_dampened("./input/day_02.txt");
    println!("safe reports dampened: {}", safe_reports);
}


struct Report{
    levels: Vec<usize>
}

trait IsSafe {
    fn is_safe(&self) -> bool;
}

impl IsSafe for Vec<usize> {
    fn is_safe(&self) -> bool {
        let consecutive_ascending = self.windows(2).all(|pair| pair[0] < pair[1]);
        let consecutive_descending = self.windows(2).all(|pair| pair[0] > pair[1]);
        let small_increase = self.windows(2).all(|pair: &[usize]| pair[0].abs_diff(pair[1]) > 0 && pair[0].abs_diff(pair[1]) <= 3);
        small_increase && (consecutive_ascending || consecutive_descending)
    }
}

impl Report {
    pub fn is_safe(&self) -> bool {
        self.levels.is_safe()
    }

    pub fn is_safe_dampened(&self) -> bool {
        if self.is_safe() {
            return true;
        }
        for (i, _) in self.levels.iter().enumerate() {
            let mut clone = self.levels.clone();
            clone.remove(i);
            if clone.is_safe() {
                return true;
            }
        }

        false
    }
}

impl FromIterator<usize> for Report {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        Report {
            levels: Vec::from_iter(iter)
        }
    }
}

fn read_report(path: &str) -> Vec<Report> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let levels = line.split_whitespace().map(|val| val.parse().unwrap());
            Report::from_iter(levels)
        })
        .collect()
}

fn count_safe_reports(path: &str) -> usize {
    read_report(path).iter().filter(|report| report.is_safe()).count()
}

fn count_safe_reports_dampened(path: &str) -> usize {
    read_report(path).iter().filter(|report| report.is_safe_dampened()).count()
}



#[cfg(test)]
mod tests {
    use crate::day_02::*;

    #[test]
    fn test_part_1() {
        assert_eq!(count_safe_reports("./input_test/day_02.txt"), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(count_safe_reports_dampened("./input_test/day_02.txt"), 4);
    }
}