advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for (_, [f1, f2]) in re.captures_iter(input).map(|caps| caps.extract()) {
        let d1 = f1.parse::<u32>().unwrap();
        let d2 = f2.parse::<u32>().unwrap();
        sum += d1 * d2;
    }
    Some(sum)
}

#[derive(PartialEq, Eq)]
enum MulState {
    Do,
    Dont,
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(don't|do)").unwrap();
    let re2 = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    let mut state = MulState::Do;
    for m in re.find_iter(input) {
        let a = re2.captures(m.as_str());
        if let Some(b) = a {
            let (_, [f1, f2]) = b.extract();
            if state == MulState::Do {
                let d1 = f1.parse::<u32>().unwrap();
                let d2 = f2.parse::<u32>().unwrap();
                sum += d1 * d2;
            }
        } else if m.as_str().starts_with("don") {
            state = MulState::Dont;
        } else {
            state = MulState::Do;
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
