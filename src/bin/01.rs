advent_of_code::solution!(1);

fn get_vecs(input: &str) -> (std::vec::Vec<i32>, std::vec::Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.split_terminator('\n') {
        let li = line.split_once(' ');
        let (l, r) = li.unwrap();
        left.push(l.trim().parse::<i32>().unwrap());
        right.push(r.trim().parse::<i32>().unwrap());
    }
    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = get_vecs(input);
    left.sort();
    right.sort();

    let mut distance: i32 = 0;

    while !left.is_empty() && !right.is_empty() {
        let a = left.pop().unwrap();
        let b = right.pop().unwrap();
        let d = (a - b).abs();
        distance += d;
    }
    Some(distance as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = get_vecs(input);
    let mut scores: i32 = 0;

    for num in left {
        let count: i32 = right.iter().filter(|x| **x == num).sum();
        scores += count;
    }
    Some(scores as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
