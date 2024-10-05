advent_of_code::solution!(12);

use regex::Regex;
use serde_json::{Map, Value};

const FORBIDDEN_RED: &str = "red";

fn obj_has_red_value(obj: &Map<String, Value>) -> bool {
    obj.values()
        .filter(|v| Value::is_string(v))
        .any(|v| v == FORBIDDEN_RED)
}

fn recursive_sum(value: &Value, cur_sum: i32) -> Option<i32> {
    let value_sum = match value {
        Value::Number(num) => num.as_i64()? as i32,
        Value::Object(obj) => {
            if !obj_has_red_value(obj) {
                obj.values()
                    .map(|v| recursive_sum(v, cur_sum).unwrap_or(0))
                    .sum()
            } else {
                0
            }
        }
        Value::Array(arr) => arr
            .iter()
            .map(|v| recursive_sum(v, cur_sum).unwrap_or(0))
            .sum(),
        _ => 0,
    };
    cur_sum.checked_add(value_sum)
}

pub fn part_one(input: &str) -> Option<i32> {
    // Get all numbers from raw string
    let re = Regex::new(r"(-?\d+)").unwrap();
    let mut results = vec![];
    for (_, [num]) in re.captures_iter(input).map(|c| c.extract()) {
        results.push(num.parse::<i32>().ok()?);
    }
    Some(results.into_iter().sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    // Parse to JSON and sum values
    let json: Value = serde_json::from_str(input).expect("Bad formatted JSON str");
    recursive_sum(&json, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
