advent_of_code::solution!(6);

use ndarray::{prelude::*, Array, Ix2};

struct Pair(i32, i32);

fn parse_instructions(instruction: &str) -> Option<(String, Pair, Pair)> {
    // Unwind instruction
    let instruction = instruction.replace("turn ", "").to_string();
    let mut elems = instruction.split_whitespace();
    let action = elems.next()?.to_string();
    let mut p1 = elems.next()?.split(",");
    let _ = elems.next()?;
    let mut p2 = elems.next()?.split(",");
    //
    // Unwind points
    let x1: i32 = p1.next()?.parse().ok()?;
    let y1: i32 = p1.next()?.parse().ok()?;
    let x2: i32 = p2.next()?.parse().ok()?;
    let y2: i32 = p2.next()?.parse().ok()?;

    Some((action, Pair(x1, y1), Pair(x2, y2)))
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut matrix = Array::<bool, Ix2>::from_elem((1000, 1000), false);

    for instruction in input.trim().split('\n') {
        // Unwind instruction
        let (action, p1, p2) = parse_instructions(instruction)?;

        // Index slice as mut
        let mut slice = matrix.slice_mut(s![p1.0..=p2.0, p1.1..=p2.1]);

        // Operate the action
        match action.as_str() {
            "off" => slice.fill(false),
            "on" => slice.fill(true),
            "toggle" => slice.map_inplace(|v| *v ^= true),
            _ => (),
        }
    }
    Some(matrix.mapv(u32::from).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut matrix = Array::<u32, Ix2>::from_elem((1000, 1000), 0);
    let mut overflow = 0;

    for instruction in input.trim().split('\n') {
        // Unwind instruction
        let (action, p1, p2) = parse_instructions(instruction)?;

        // Index slice as mut
        let mut slice = matrix.slice_mut(s![p1.0..=p2.0, p1.1..=p2.1]);

        // Operate the action
        match action.as_str() {
            "off" => slice.mapv_inplace(|v| v.saturating_sub(1)),
            "on" => slice.mapv_inplace(|v| {
                if let Some(a) = v.checked_add(1) {
                    a
                } else {
                    overflow += 1;
                    v
                }
            }),
            "toggle" => slice.mapv_inplace(|v| {
                if let Some(a) = v.checked_add(2) {
                    a
                } else {
                    overflow += 2;
                    v
                }
            }),
            _ => (),
        }
    }
    Some(matrix.sum() + overflow)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(73));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(83));
    }
}
