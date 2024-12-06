use array2d::Array2D;
use rayon::prelude::*;
use rustc_hash::FxHashSet;

advent_of_code::solution!(6);

#[derive(Clone, PartialEq)]
enum Squares {
    Blocked,
    Free,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn next_pos(&self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => {
                let (r, c) = pos;
                (r - 1, c)
            }
            Direction::Down => {
                let (r, c) = pos;
                (r + 1, c)
            }
            Direction::Left => {
                let (r, c) = pos;
                (r, c - 1)
            }
            Direction::Right => {
                let (r, c) = pos;
                (r, c + 1)
            }
        }
    }
}

struct Guard {
    pos: (i32, i32),
    direction: Direction,
    visited: FxHashSet<(i32, i32)>,
    visits: FxHashSet<(i32, i32, Direction)>,
    loop_detected: bool,
}

impl Guard {
    fn new(start_pos: (i32, i32)) -> Self {
        Guard {
            pos: start_pos,
            direction: Direction::Up,
            visited: vec![start_pos].into_iter().collect(),
            visits: vec![].into_iter().collect(),
            loop_detected: false,
        }
    }
    fn step(&mut self, arr: &Array2D<Squares>) -> bool {
        let next_pos = self.direction.next_pos(self.pos);
        let (r, c) = next_pos;
        if r >= arr.num_rows() as i32 || r < 0 {
            return false;
        }
        if c >= arr.num_columns() as i32 || c < 0 {
            return false;
        }

        if !self.visits.insert((self.pos.0, self.pos.1, self.direction)) {
            self.loop_detected = true;
            return false;
        }

        if *arr.get(next_pos.0 as usize, next_pos.1 as usize).unwrap() == Squares::Blocked {
            self.direction = self.direction.turn();
        } else {
            self.pos = next_pos;
            self.visited.insert(self.pos);
        }

        true
    }
}

fn parse_arr(input: &str) -> ((i32, i32), Array2D<Squares>) {
    let rows = input.split_terminator('\n').count() as i32;
    let cols = input.split_terminator('\n').next().unwrap().len() as i32;
    let lines = input.split_terminator('\n');
    let mut arr = Array2D::filled_with(Squares::Free, rows as usize, cols as usize);

    let mut start = (0_i32, 0_i32);

    for (row, line) in lines.enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                arr.set(row, col, Squares::Blocked).unwrap();
            } else if ch == '^' {
                start = (row as i32, col as i32);
            }
        }
    }

    (start, arr)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (start, arr) = parse_arr(input);

    let mut guard = Guard::new(start);
    while guard.step(&arr) {}

    Some(guard.visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (start, arr) = parse_arr(input);
    let mut guard = Guard::new(start);
    while guard.step(&arr) {}

    let loops = guard
        .visited
        .par_iter()
        .filter(|a| {
            let mut guard2 = Guard::new(start);
            let mut new_arr = arr.clone();
            new_arr
                .set(a.0 as usize, a.1 as usize, Squares::Blocked)
                .unwrap();
            while guard2.step(&new_arr) {}

            guard2.loop_detected
        })
        .count();

    Some(loops as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }
    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(5516));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2008));
    }
}
