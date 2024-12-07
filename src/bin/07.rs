advent_of_code::solution!(7);

#[derive(Clone)]
struct Row {
    result: u64,
    values: Vec<u64>,
}

fn parse(input: &str) -> Vec<Row> {
    let mut rows = Vec::new();
    for line in input.split_terminator('\n') {
        let (result, values) = line.split_once(':').unwrap();
        let values = values
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let result = result.parse::<u64>().unwrap();

        rows.push(Row { result, values });
    }

    rows
}

fn get_result(
    nums: &[u64],
    current_total: u64,
    operations: &[fn(u64, u64) -> u64],
    expected: u64,
) -> bool {
    if nums.is_empty() {
        return current_total == expected;
    }
    if current_total > expected {
        return false;
    }

    let a = current_total;
    let b = nums[0];
    for op in operations {
        let result = op(a, b);
        if get_result(&nums[1..], result, operations, expected) {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let rows = parse(input);

    let operations = [
        |a: u64, b: u64| -> u64 { a + b },
        |a: u64, b: u64| -> u64 { a * b },
    ];

    let sum = rows
        .iter()
        .map(|r| {
            if get_result(&r.values[..], 0, &operations[..], r.result) {
                return r.result;
            }
            0
        })
        .reduce(|a, b| a + b)
        .unwrap();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rows = parse(input);

    let operations = [
        |a: u64, b: u64| -> u64 { a + b },
        |a: u64, b: u64| -> u64 { a * b },
        |a: u64, b: u64| -> u64 { (a * 10u64.pow(b.ilog10() + 1)) + b },
    ];

    let sum = rows
        .iter()
        .map(|r| {
            if get_result(&r.values[..], 0, &operations[..], r.result) {
                return r.result;
            }
            0
        })
        .reduce(|a, b| a + b)
        .unwrap();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
