advent_of_code::solution!(8);

use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use rustc_hash::{FxHashMap, FxHashSet};

use advent_of_code::helper::grid_size;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}
impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point {
            x: x as i32,
            y: y as i32,
        }
    }

    fn diff(&self, rhs: &Point) -> Point {
        let x = (self.x - rhs.x).abs();
        let y = (self.y - rhs.y).abs();
        Point { x, y }
    }

    fn is_within(&self, x_lim: i32, y_lim: i32) -> bool {
        self.x >= 0 && self.x < x_lim && self.y >= 0 && self.y < y_lim
    }

    fn mirror(&self, point: &Point, diff: &Point) -> Point {
        let x = match self.x.cmp(&point.x) {
            std::cmp::Ordering::Less => self.x - diff.x,
            std::cmp::Ordering::Equal => self.x,
            std::cmp::Ordering::Greater => self.x + diff.x,
        };
        let y = match self.y.cmp(&point.y) {
            std::cmp::Ordering::Less => self.y - diff.y,
            std::cmp::Ordering::Equal => self.y,
            std::cmp::Ordering::Greater => self.y + diff.y,
        };
        Point { x, y }
    }
}

fn get_antennas(input: &str) -> FxHashMap<char, Vec<Point>> {
    let mut antennas: FxHashMap<char, Vec<Point>> = FxHashMap::default();
    for (y, line) in input.split_terminator('\n').enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char.is_alphanumeric() {
                let p = Point::new(x, y);
                antennas
                    .entry(char)
                    .and_modify(|v| v.push(p))
                    .or_insert(vec![p]);
            }
        }
    }
    antennas
}

pub fn part_one(input: &str) -> Option<u32> {
    let (x, y) = grid_size(input);
    let antennas = get_antennas(input);
    let mut rfs = FxHashSet::default();
    for (_freq, positions) in antennas {
        for (i, pos1) in positions.iter().enumerate() {
            for pos2 in &positions[i + 1..] {
                let diff = pos1.diff(pos2);
                println!();
                let sub = pos1.mirror(pos2, &diff);
                let add = pos2.mirror(pos1, &diff);

                if sub.is_within(x as i32, y as i32) {
                    rfs.insert(sub);
                }
                if add.is_within(x as i32, y as i32) {
                    rfs.insert(add);
                }
            }
        }
    }

    Some(rfs.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (x, y) = grid_size(input);
    let antennas = get_antennas(input);
    let mut rfs = FxHashSet::default();
    for (_freq, positions) in antennas {
        for (i, pos1) in positions.iter().enumerate() {
            rfs.insert(*pos1);
            for pos2 in &positions[i + 1..] {
                rfs.insert(*pos2);
                let diff = pos1.diff(pos2);

                let mut start = *pos1;
                let mut mirr = *pos2;
                loop {
                    let sub = start.mirror(&mirr, &diff);
                    if sub.is_within(x as i32, y as i32) {
                        rfs.insert(sub);
                    } else {
                        break;
                    }
                    mirr = start;
                    start = sub;
                }

                let mut start = *pos2;
                let mut mirr = *pos1;
                loop {
                    let sub = start.mirror(&mirr, &diff);
                    if sub.is_within(x as i32, y as i32) {
                        rfs.insert(sub);
                    } else {
                        break;
                    }
                    mirr = start;
                    start = sub;
                }
            }
        }
    }
    Some(rfs.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(392));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1235));
    }
}
