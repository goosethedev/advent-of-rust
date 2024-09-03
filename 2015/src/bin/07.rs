use std::collections::HashMap;

advent_of_code::solution!(7);

type WireMap = HashMap<String, SignalType>;

enum SignalType {
    Send(String),
    And(String, String),
    Or(String, String),
    Not(String),
    LShift(String, u8),
    RShift(String, u8),
}

fn build_wiremap(instructions: &str) -> Option<WireMap> {
    let mut signals = WireMap::new();
    for instruction in instructions.lines() {
        let mut elems = instruction.trim().split(" -> ");
        let instruction = elems.next()?;
        let target = elems.next()?.to_string();
        let operand = parse_instructions(instruction)?;
        signals.insert(target, operand);
    }
    Some(signals)
}

fn parse_instructions(instruction: &str) -> Option<SignalType> {
    let mut elems = instruction.split_whitespace();
    let op1 = elems.next()?.to_string();

    // Return the next param if NOT
    if op1 == "NOT" {
        return Some(SignalType::Not(elems.next()?.into()));
    };

    // If there's a following element, it's an action. Match it.
    if let Some(op2) = elems.next() {
        let signal_type = match op2 {
            "AND" => SignalType::And(op1, elems.next()?.into()),
            "OR" => SignalType::Or(op1, elems.next()?.into()),
            "LSHIFT" => SignalType::LShift(op1, elems.next()?.parse().ok()?),
            "RSHIFT" => SignalType::RShift(op1, elems.next()?.parse().ok()?),
            _ => return None,
        };
        Some(signal_type)
    } else {
        // Single element, send it directly
        Some(SignalType::Send(op1))
    }
}

fn get_signal(wire: &str, map: &WireMap, cache: &mut HashMap<String, u16>) -> u16 {
    if cache.contains_key(wire) {
        return cache[wire];
    }

    let mut resolve = |val: &str| -> u16 {
        if let Ok(parsed) = val.parse::<u16>() {
            parsed
        } else {
            get_signal(val, map, cache)
        }
    };

    let value = match map.get(wire).unwrap() {
        SignalType::Send(op) => resolve(op),
        SignalType::And(op1, op2) => resolve(op1) & resolve(op2),
        SignalType::Or(op1, op2) => resolve(op1) | resolve(op2),
        SignalType::Not(op) => !resolve(op),
        SignalType::LShift(op, bits) => resolve(op) << bits,
        SignalType::RShift(op, bits) => resolve(op) >> bits,
    };
    cache.insert(wire.into(), value);
    value
}

pub fn part_one(input: &str) -> Option<u16> {
    let signals = build_wiremap(input)?;
    let mut cache = HashMap::new();
    Some(get_signal("a", &signals, &mut cache))
}

pub fn part_two(input: &str) -> Option<u16> {
    let a = format!("{}", part_one(input)?);
    let mut signals = build_wiremap(input)?;
    signals.insert("b".into(), SignalType::Send(a));
    let mut cache = HashMap::new();
    Some(get_signal("a", &signals, &mut cache))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }
}
