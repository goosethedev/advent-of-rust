advent_of_code::solution!(10);

// TODO: Reimplement this to make it more efficient/concise
fn look_and_say(input: &str) -> String {
    let mut chars = input.chars();
    let mut prev = chars.next().unwrap();
    let mut count = 1;
    let mut result = "".to_string();
    loop {
        if let Some(ch) = chars.next() {
            if prev == ch {
                count += 1;
            } else {
                result.push_str(&format!("{count}"));
                result.push(prev);
                prev = ch;
                count = 1;
            }
        } else {
            result.push_str(&format!("{count}"));
            result.push(prev);
            break result;
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input = input.trim().to_owned();
    for _ in 0..40 {
        input = look_and_say(&input);
    }
    input.len().try_into().ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = input.trim().to_owned();
    for _ in 0..50 {
        input = look_and_say(&input);
    }
    input.len().try_into().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_and_say() {
        let result = look_and_say("1");
        assert_eq!(result, "11");
        let result = look_and_say(&result);
        assert_eq!(result, "21");
        let result = look_and_say(&result);
        assert_eq!(result, "1211");
        let result = look_and_say(&result);
        assert_eq!(result, "111221");
        let result = look_and_say(&result);
        assert_eq!(result, "312211");
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82350));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1166642));
    }
}
