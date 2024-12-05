use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(5);

type OrderMap = HashMap<u32, Vec<u32>>;
type Updates = Vec<Vec<u32>>;

fn parse(input: &str) -> (OrderMap, Updates) {
    let mut order: OrderMap = HashMap::new();
    let mut updates: Updates = Vec::new();
    for line in input.split_terminator('\n') {
        if line.len() == 5 {
            let (l, r) = line.split_once('|').unwrap();
            let ll = l.parse::<u32>().unwrap();
            let rr = r.parse::<u32>().unwrap();
            if let Some(v) = order.get_mut(&ll) {
                v.push(rr);
            } else {
                let v = vec![rr];
                order.insert(ll, v);
            }
        } else if line.len() > 5 {
            let mut update = Vec::new();
            for snum in line.split(',') {
                let num = snum.parse::<u32>().unwrap();
                update.push(num);
            }
            updates.push(update);
        }
    }
    (order, updates)
}

fn get_valid_invalid(order: &OrderMap, updates: Updates) -> (Updates, Updates) {
    let mut valid_updates: Updates = Vec::new();
    let mut invalid_updates: Updates = Vec::new();
    for update in &updates {
        let mut valid = true;
        'outer: for (pos, page) in update.iter().enumerate() {
            for post_page in &update[pos + 1..] {
                if let Some(k) = order.get(page) {
                    if !k.contains(post_page) {
                        valid = false;
                        break 'outer;
                    }
                }

                if let Some(k) = order.get(post_page) {
                    if k.contains(page) {
                        valid = false;
                        break 'outer;
                    }
                }
            }
        }
        if valid {
            valid_updates.push(update.clone());
        } else {
            invalid_updates.push(update.clone());
        }
    }

    (valid_updates, invalid_updates)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (order, updates) = parse(input);
    let (valid_updates, _invalid_updates) = get_valid_invalid(&order, updates);
    let mut sum = 0;
    for valid in valid_updates {
        sum += valid[valid.len() / 2];
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (order, updates) = parse(input);
    let (_valid_updates, mut invalid_updates) = get_valid_invalid(&order, updates);

    let mut sum = 0;

    for update in invalid_updates.iter_mut() {
        update.sort_by(|a, b| {
            if let Some(v) = order.get(a) {
                if v.contains(b) {
                    return Ordering::Greater;
                }
            }
            Ordering::Less
        });

        sum += update[update.len() / 2];
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6505));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6897));
    }
}
