use std::collections::HashMap;

advent_of_code::solution!(5);

// Join two characters as a two-letter String
fn get_pair(char1: char, char2: char) -> String {
    format!("{char1}{char2}")
}

fn is_nice_part_one(input: &str) -> bool {
    let mut vocals = 0;
    let mut double_letter = false;
    let disallowed = ["ab", "cd", "pq", "xy"];
    let mut prev = '\n'; // placeholder
    for ch in input.trim().chars() {
        if prev != '\n' {
            // Check if pair is disallowed
            if disallowed.contains(&get_pair(prev, ch).as_str()) {
                return false;
            }
            // Set if letter is repeated
            double_letter = double_letter || prev == ch;
        };
        // Count vocals
        if "aeiou".contains(ch) {
            vocals += 1;
        }
        prev = ch;
    }
    vocals >= 3 && double_letter
}

fn is_nice_part_two(input: &str) -> bool {
    let mut pair_twice = false;
    let mut pair_letter_between = false;
    let mut pairs_pos: HashMap<String, u8> = HashMap::new();
    let mut curr_pos = 0;

    // Setup window
    let mut chars = input.chars();
    let Some(mut ch1) = chars.next() else {
        return false;
    };
    let Some(mut ch2) = chars.next() else {
        return false;
    };

    for ch3 in chars {
        let pair = get_pair(ch1, ch2);
        match pairs_pos.get(&pair) {
            Some(found_pos) => {
                pair_twice = pair_twice || (curr_pos - found_pos) > 1;
            }
            None => {
                pairs_pos.insert(pair, curr_pos);
            }
        }
        pair_letter_between = pair_letter_between || (ch1 == ch3);

        // Both requisites fulfilled
        if pair_twice && pair_letter_between {
            return true;
        }

        // Setup next iteration
        (ch1, ch2) = (ch2, ch3);
        curr_pos += 1;
    }
    // ChecK last pair left unchecked
    match pairs_pos.get(&get_pair(ch1, ch2)) {
        Some(found_pos) => (pair_twice || (curr_pos - found_pos) > 1) && pair_letter_between,
        None => false,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .split('\n')
            .map(|line| is_nice_part_one(line) as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .split('\n')
            .map(|line| is_nice_part_two(line) as u32)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nice_part_one() {
        assert!(is_nice_part_one("aaa"));
        assert!(is_nice_part_one("ugknbfddgicrmopn"));
        assert!(!is_nice_part_one("jchzalrnumimnmhp"));
        assert!(!is_nice_part_one("haegwjzuvuyypxyu"));
        assert!(!is_nice_part_one("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_is_nice_part_two() {
        assert!(is_nice_part_two("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_part_two("xxyxx"));
        assert!(is_nice_part_two("xxxx")); // Non-overlapping
        assert!(!is_nice_part_two("xxx")); // Overlapping
        assert!(!is_nice_part_two("xx"));
        assert!(!is_nice_part_two("x"));
        assert!(!is_nice_part_two("stgmbstg"));
        assert!(!is_nice_part_two("uurcxstgmygtbstg"));
        assert!(!is_nice_part_two("ieodomkazucvgmuy"));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    #[ignore = "nah"]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
