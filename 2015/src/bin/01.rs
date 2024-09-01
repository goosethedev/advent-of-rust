advent_of_code::solution!(1);

fn move_floor(cur_floor: i64, ch: char) -> Option<i64> {
    match ch {
        '(' => cur_floor.checked_add(1),
        ')' => cur_floor.checked_sub(1),
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let floor = input
        .trim()
        .chars()
        .fold(0, |acc, ch| move_floor(acc, ch).unwrap());
    Some(floor)

    // Alternative answer

    // time: 48us
    // input
    //     .matches('(')
    //     .count()
    //     .checked_mul(2)?
    //     .checked_sub(input.len())?
    //     .try_into()
    //     .ok()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut floor: i64 = 0;
    for (i, ch) in input.char_indices() {
        floor = move_floor(floor, ch)?;
        if floor == -1 {
            return i.checked_add(1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(-1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
