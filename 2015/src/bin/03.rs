use std::collections::HashSet;

advent_of_code::solution!(3);

fn move_by_coord(mut x: i32, mut y: i32, ch: char) -> (i32, i32) {
    match ch {
        '^' => y += 1,
        'v' => y -= 1,
        '>' => x += 1,
        '<' => x -= 1,
        _ => (),
    }
    (x, y)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = HashSet::new();
    // First house always gets in
    let mut houses = 1;
    let (mut x, mut y) = (0, 0);
    map.insert((0, 0));
    for dir in input.chars() {
        (x, y) = move_by_coord(x, y, dir);
        // Add if coords not in map
        houses += !map.contains(&(x, y)) as u32;
        map.insert((x, y));
    }
    Some(houses)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = HashSet::new();
    // First house always gets in
    let mut houses = 1;
    let (mut x1, mut y1) = (0, 0);
    let (mut x2, mut y2) = (0, 0);
    map.insert((0, 0));
    for (i, dir) in input.char_indices() {
        let (x, y) = if i % 2 == 0 {
            (x1, y1) = move_by_coord(x1, y1, dir);
            (x1, y1)
        } else {
            (x2, y2) = move_by_coord(x2, y2, dir);
            (x2, y2)
        };
        // Add if coords not in map
        houses += !map.contains(&(x, y)) as u32;
        map.insert((x, y));
    }
    Some(houses)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
