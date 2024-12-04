use array2d::Array2D;

advent_of_code::solution!(4);

fn check(arr: &Array2D<char>, pos: &[(usize, usize)]) -> bool {
    arr.get(pos[0].0, pos[0].1).unwrap_or(&'1') == &'X'
        && arr.get(pos[1].0, pos[1].1).unwrap_or(&'1') == &'M'
        && arr.get(pos[2].0, pos[2].1).unwrap_or(&'1') == &'A'
        && arr.get(pos[3].0, pos[3].1).unwrap_or(&'1') == &'S'
}

fn check_horizontal_forward(arr: &Array2D<char>, pos: (usize, usize)) -> bool {
    let (i, j) = pos;
    let v = vec![(i, j), (i, j + 1), (i, j + 2), (i, j + 3)];
    check(arr, &v)
}

fn check_horizontal_backward(arr: &Array2D<char>, pos: (usize, usize)) -> bool {
    let (i, j) = pos;
    let v = vec![
        (i, j),
        (i, j.checked_sub(1).unwrap_or(10000)),
        (i, j.checked_sub(2).unwrap_or(10000)),
        (i, j.checked_sub(3).unwrap_or(10000)),
    ];
    check(arr, &v)
}

fn check_vertical_down(arr: &Array2D<char>, pos: (usize, usize)) -> bool {
    let (i, j) = pos;
    let v = vec![(i, j), (i + 1, j), (i + 2, j), (i + 3, j)];
    check(arr, &v)
}

fn check_vertical_up(arr: &Array2D<char>, pos: (usize, usize)) -> bool {
    let (i, j) = pos;
    let v = vec![
        (i, j),
        (i.checked_sub(1).unwrap_or(10000), j),
        (i.checked_sub(2).unwrap_or(10000), j),
        (i.checked_sub(3).unwrap_or(10000), j),
    ];
    check(arr, &v)
}

fn check_diagonal_down_right(arr: &Array2D<char>, pos: (usize, usize)) -> bool {
    let (i, j) = pos;
    let v = vec![(i, j), (i + 1, j + 1), (i + 2, j + 2), (i + 3, j + 3)];
    check(arr, &v)
}

fn check_diagonal_up_right(arr: &Array2D<char>, pos: (usize, usize)) -> bool {
    let (i, j) = pos;
    let v = vec![
        (i, j),
        (i.checked_sub(1).unwrap_or(10000), j + 1),
        (i.checked_sub(2).unwrap_or(10000), j + 2),
        (i.checked_sub(3).unwrap_or(10000), j + 3),
    ];
    // println!("{:?}", v);
    check(arr, &v)
}

fn check_diagonal_down_left(arr: &Array2D<char>, pos: (usize, usize)) -> bool {
    let (i, j) = pos;
    let v = vec![
        (i, j),
        (i + 1, j.checked_sub(1).unwrap_or(10000)),
        (i + 2, j.checked_sub(2).unwrap_or(10000)),
        (i + 3, j.checked_sub(3).unwrap_or(10000)),
    ];
    check(arr, &v)
}

fn check_diagonal_up_left(arr: &Array2D<char>, pos: (usize, usize)) -> bool {
    let (i, j) = pos;
    let v = vec![
        (i, j),
        (
            i.checked_sub(1).unwrap_or(10000),
            j.checked_sub(1).unwrap_or(10000),
        ),
        (
            i.checked_sub(2).unwrap_or(10000),
            j.checked_sub(2).unwrap_or(10000),
        ),
        (
            i.checked_sub(3).unwrap_or(10000),
            j.checked_sub(3).unwrap_or(10000),
        ),
    ];
    check(arr, &v)
}

fn create_matrix(input: &str) -> Array2D<char> {
    let mut rows = Vec::new();
    for line in input.split_terminator('\n') {
        let s: Vec<char> = line.chars().collect();
        rows.push(s);
    }
    Array2D::from_rows(&rows).unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let arr = create_matrix(input);

    let mut sum_xmas = 0;
    for ((i, j), _c) in arr.enumerate_row_major() {
        // println!("{} {} {}", i, j, _c);
        let pos = (i, j);
        if check_horizontal_forward(&arr, pos) {
            sum_xmas += 1;
        }
        if check_horizontal_backward(&arr, pos) {
            sum_xmas += 1;
        }
        if check_diagonal_up_right(&arr, pos) {
            sum_xmas += 1;
        }
        if check_diagonal_down_right(&arr, pos) {
            sum_xmas += 1;
        }
        if check_diagonal_down_left(&arr, pos) {
            sum_xmas += 1;
        }
        if check_diagonal_up_left(&arr, pos) {
            sum_xmas += 1;
        }
        if check_vertical_down(&arr, pos) {
            sum_xmas += 1;
        }
        if check_vertical_up(&arr, pos) {
            sum_xmas += 1;
        }
    }

    Some(sum_xmas)
}

fn check_mas2(arr: &Array2D<char>, pos: ((usize, usize), (usize, usize))) -> bool {
    if arr.get(pos.0 .0, pos.0 .1).unwrap_or(&'1') == &'M'
        && arr.get(pos.1 .0, pos.1 .1).unwrap_or(&'1') == &'S'
    {
        return true;
    }

    if arr.get(pos.0 .0, pos.0 .1).unwrap_or(&'1') == &'S'
        && arr.get(pos.1 .0, pos.1 .1).unwrap_or(&'1') == &'M'
    {
        return true;
    }

    false
}

fn check_mas(arr: &Array2D<char>, pos: (usize, usize)) -> bool {
    let (i, j) = pos;
    let v1 = (
        (
            i.checked_sub(1).unwrap_or(10000),
            j.checked_sub(1).unwrap_or(10000),
        ),
        (i + 1, j + 1),
    );
    let v2 = (
        (i.checked_sub(1).unwrap_or(10000), j + 1),
        (i + 1, j.checked_sub(1).unwrap_or(10000)),
    );

    if arr.get(i, j).unwrap_or(&'1') == &'A' {
        if check_mas2(arr, v1) && check_mas2(arr, v2) {
            return true;
        }
    }

    false
}

pub fn part_two(input: &str) -> Option<u32> {
    let arr = create_matrix(input);
    let mut sum = 0;
    for ((i, j), _c) in arr.enumerate_row_major() {
        let pos = (i, j);
        if check_mas(&arr, pos) {
            sum += 1;
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
