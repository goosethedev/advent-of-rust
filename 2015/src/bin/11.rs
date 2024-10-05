use core::str;

advent_of_code::solution!(11);

const FORBIDDEN_CHARS: [u8; 3] = [b'i', b'o', b'l'];

fn remove_forbidden(arr: &[u8; 8]) -> [u8; 8] {
    let mut found = false;
    arr.into_iter()
        .map(|ch| {
            if found {
                b'a'
            } else if FORBIDDEN_CHARS.contains(ch) {
                found = true;
                ch.checked_add(1).unwrap()
            } else {
                *ch
            }
        })
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap()
}

fn increase_one(mut arr: [u8; 8]) -> [u8; 8] {
    let mut n = arr.len() - 1;
    while n > 0 {
        if arr[n] == b'z' {
            arr[n] = b'a';
            n -= 1;
        } else {
            arr[n] += 1;
            break;
        }
    }
    return arr;
}

fn pass_conditions(arr: &[u8; 8]) -> bool {
    // No forbidden chars
    let no_forbidden = !arr.iter().any(|v| FORBIDDEN_CHARS.contains(v));

    // Two non-overlapping pairs
    let mut count_pairs = 0;
    let mut iter = arr.iter();
    let mut curr = iter.next().unwrap();
    loop {
        let mut next = if let Some(next) = iter.next() {
            next
        } else {
            break;
        };

        if curr == next {
            count_pairs += 1;
            next = if let Some(next) = iter.next() {
                next
            } else {
                break;
            };
        }

        curr = next;
    }

    // Increasing 3-char straight
    let mut three_straight = false;
    let mut iter = arr.iter();
    let mut ch1 = iter.next().unwrap();
    let mut ch2 = iter.next().unwrap();
    for ch3 in iter {
        if *ch1 + 1 == *ch2 && *ch2 + 1 == *ch3 {
            three_straight = true;
            break;
        }
        (ch1, ch2) = (ch2, ch3);
    }
    no_forbidden && count_pairs >= 2 && three_straight
}

pub fn part_one(input: &str) -> Option<String> {
    let mut input: [u8; 8] = input.trim().as_bytes().try_into().ok()?;
    while input != "zzzzzzzz".as_bytes() {
        input = remove_forbidden(&input);
        if pass_conditions(&input) {
            return String::from_utf8(input.to_vec()).ok();
        }
        input = increase_one(input);
    }
    None
}

pub fn part_two(input: &str) -> Option<String> {
    let input: [u8; 8] = part_one(&input).unwrap().as_bytes().try_into().ok()?;
    let input = String::from_utf8(increase_one(input).to_vec()).ok()?;
    part_one(&input)
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[test]
    fn test_increase_one_simple() -> Result<(), Box<dyn Error>> {
        let input: [u8; 8] = "arstqwfs".as_bytes().try_into()?;
        let result = String::from_utf8(increase_one(input).to_vec())?;
        let expected = "arstqwft".to_owned();
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_increase_one_carry() -> Result<(), Box<dyn Error>> {
        let input: [u8; 8] = "arstqwfz".as_bytes().try_into()?;
        let result = String::from_utf8(increase_one(input).to_vec())?;
        let expected = "arstqwga".to_owned();
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_increase_one_mult_carry() -> Result<(), Box<dyn Error>> {
        let input: [u8; 8] = "arstzzzz".as_bytes().try_into()?;
        let result = String::from_utf8(increase_one(input).to_vec())?;
        let expected = "arsuaaaa".to_owned();
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_conditions_unallowed() -> Result<(), Box<dyn Error>> {
        let input: [u8; 8] = "hijklmmn".as_bytes().try_into()?;
        assert!(!pass_conditions(&input));
        let input: [u8; 8] = "abbceffg".as_bytes().try_into()?;
        assert!(!pass_conditions(&input));
        let input: [u8; 8] = "abbcegjk".as_bytes().try_into()?;
        assert!(!pass_conditions(&input));
        let input: [u8; 8] = "abckkkmn".as_bytes().try_into()?;
        assert!(!pass_conditions(&input));
        let input: [u8; 8] = "ghjaaaaa".as_bytes().try_into()?;
        assert!(!pass_conditions(&input));
        Ok(())
    }

    #[test]
    fn test_conditions_allowed_1() -> Result<(), Box<dyn Error>> {
        let input: [u8; 8] = "abckkmmn".as_bytes().try_into()?;
        assert!(pass_conditions(&input));
        Ok(())
    }

    #[test]
    fn test_conditions_allowed_2() -> Result<(), Box<dyn Error>> {
        let input: [u8; 8] = "kkabcmmn".as_bytes().try_into()?;
        assert!(pass_conditions(&input));
        Ok(())
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("abcdefgh"), Some("abcdffaa".to_owned()));
        assert_eq!(part_one("ghijklmn"), Some("ghjaabcc".to_owned()));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("abcdefgh"), Some("abcdffbb".to_owned()));
        assert_eq!(part_two("ghijklmn"), Some("ghjbbcdd".to_owned()));
    }
}
