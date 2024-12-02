advent_of_code::solution!(2);

#[derive(PartialEq, Eq)]
enum Direction {
    Ascending,
    Descending,
}
struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn new(levels: Vec<i32>) -> Self {
        Self { levels }
    }

    fn branch(&self, pos: usize) -> bool {
        let mut levels1 = self.levels.clone();
        levels1.remove(pos);
        let mut r1 = Report::new(levels1);
        if r1.is_safe(false) {
            return true;
        }

        let mut levels2 = self.levels.clone();
        levels2.remove(pos + 1);
        let mut r2 = Report::new(levels2);
        if r2.is_safe(false) {
            return true;
        }
        false
    }

    fn is_safe(&mut self, branch: bool) -> bool {
        let direction = if self.levels.first().unwrap() > self.levels.last().unwrap() {
            Direction::Descending
        } else {
            Direction::Ascending
        };

        let itr = self.levels.windows(2);
        for (pos, slice) in itr.enumerate() {
            if slice.len() != 2 {
                continue;
            }
            let diff = slice[0].abs_diff(slice[1]);
            if !(1..=3).contains(&diff) {
                if branch {
                    return self.branch(pos);
                }
                return false;
            }
            if slice[0] > slice[1] && direction == Direction::Ascending {
                if branch {
                    return self.branch(pos);
                }
                return false;
            }
            if slice[0] < slice[1] && direction == Direction::Descending {
                if branch {
                    return self.branch(pos);
                }
                return false;
            }
        }

        true
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe = 0;
    for line in input.split_terminator('\n') {
        let s_levels = line.split_ascii_whitespace();
        let levels: Vec<i32> = s_levels.map(|x| x.parse::<i32>().unwrap()).collect();
        let mut report = Report::new(levels);
        if report.is_safe(false) {
            safe += 1;
        }
    }

    Some(safe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe = 0;
    for line in input.split_terminator('\n') {
        let s_levels = line.split_ascii_whitespace();
        let levels: Vec<i32> = s_levels.map(|x| x.parse::<i32>().unwrap()).collect();
        let mut report = Report::new(levels);
        if report.is_safe(true) {
            safe += 1;
        }
    }

    Some(safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(421));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(476));
    }
    #[test]
    fn test_abs() {
        assert_eq!(3_u32.abs_diff(6), 3);
        assert_eq!(3_u32.abs_diff(4), 1);
        assert_eq!(4_u32.abs_diff(3), 1);
        assert_eq!(6_u32.abs_diff(3), 3);
    }
}
