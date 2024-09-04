advent_of_code::solution!(8);

// Escaped sequences: \\ \" \xAB

fn count_in_mem(input: &str) -> u32 {
    let mut iter = input.chars();
    let mut count = 0;
    while let Some(ch) = iter.next() {
        if ch == '\\' && iter.next().unwrap() == 'x' {
            iter.nth(1); // skips the next 2 chars
        }
        count += 1;
    }
    // Ignore surrounding quotes
    count - 2
}

fn expanded_in_mem(input: &str) -> u32 {
    // Count \ or " as two characters, otherwise one
    let char_counter = |ch: char| 1 + (ch == '\\' || ch == '\"') as u32;
    input.chars().map(char_counter).sum::<u32>() + 2 // Add surrounding quotes
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut diff: u32 = 0;
    for line in input.trim().split("\n") {
        let total: u32 = line.len().try_into().ok()?;
        diff += total.checked_sub(count_in_mem(line))?;
    }
    Some(diff)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut diff: u32 = 0;
    for line in input.trim().split("\n") {
        let total: u32 = line.len().try_into().ok()?;
        diff += expanded_in_mem(line).checked_sub(total)?;
    }
    Some(diff)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expanded_in_mem() {
        assert_eq!(expanded_in_mem("\"\""), 6);
        assert_eq!(expanded_in_mem("\"abc\""), 9);
        assert_eq!(expanded_in_mem("\"aaa\\\"aaa\""), 16);
        assert_eq!(expanded_in_mem("\"\\x27\""), 11);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19));
    }
}
