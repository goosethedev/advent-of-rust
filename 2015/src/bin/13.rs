use std::collections::HashMap;

advent_of_code::solution!(13);

type Dict = HashMap<String, Relations>;
type Relations = HashMap<String, i32>;

fn parse_line(line: &str) -> Option<(String, String, i32)> {
    let elems: Vec<String> = line
        .trim()
        .replace(".", "")
        .split_whitespace()
        .map(str::to_string)
        .collect();
    let from = elems[0].clone();
    let sign = match elems[2].as_str() {
        "gain" => 1,
        "lose" => -1,
        _ => return None,
    };
    let weight = elems[3].parse::<i32>().unwrap();
    let to = elems[elems.len() - 1].replace(".", "").to_string();

    Some((from, to, sign * weight))
}

fn happiness_together(dict: &Dict, p1: &str, p2: &str) -> i32 {
    let forward = dict.get(p1).unwrap().get(p2).unwrap();
    let backward = dict.get(p2).unwrap().get(p1).unwrap();
    forward + backward
}

fn find_max_weight_sum(
    dict: &Dict,
    cur_person: &String,
    taken: &mut Vec<String>,
    cur_sum: i32,
) -> i32 {
    let relations = dict
        .get(cur_person)
        .unwrap()
        .iter()
        .filter(|rel| !taken.contains(rel.0));

    let mut max_weight = i32::MIN;
    let mut next_person = None;

    for rel in relations {
        let total = happiness_together(dict, cur_person, rel.0);
        if total >= max_weight {
            max_weight = total;
            next_person = Some(rel.0);
        }
    }

    if let Some(next_person) = next_person {
        taken.push(cur_person.to_string());
        find_max_weight_sum(dict, next_person, taken, cur_sum + max_weight)
    } else {
        // Add the happiness of the first and last person sit together
        cur_sum + happiness_together(dict, cur_person, &taken[0].to_string())
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut people: Dict = HashMap::new();

    // Parsing of input lines
    for line in input.trim().lines() {
        let (from, to, weight) = parse_line(line).unwrap();

        if let Some(person) = people.get_mut(&from) {
            person.insert(to, weight);
        } else {
            let mut relations = Relations::new();
            relations.insert(to, weight);
            people.insert(from, relations);
        };
    }

    // Finding max happiness
    let mut max_happiness = i32::MIN;
    for name in people.keys() {
        let mut taken = vec![name.clone()];
        let local_max = find_max_weight_sum(&people, name, &mut taken, 0);
        max_happiness = max_happiness.max(local_max);
    }
    Some(max_happiness)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut people: Dict = HashMap::new();

    // Adding me
    let me = "Me";
    people.insert(me.into(), Relations::new());

    // Parsing of input lines
    for line in input.trim().lines() {
        let (from, to, weight) = parse_line(line).unwrap();

        if let Some(person) = people.get_mut(&from) {
            person.insert(to, weight);
        } else {
            let mut relations = Relations::new();
            relations.insert(to, weight);
            relations.insert(me.into(), 0); // Added me on creation
            people.insert(from.clone(), relations);
            people.get_mut(me).unwrap().insert(from, 0); // Here too
        };
    }

    // Finding max happiness
    let mut max_happiness = i32::MIN;
    for name in people.keys() {
        let mut taken = vec![name.clone()];
        let local_max = find_max_weight_sum(&people, name, &mut taken, 0);
        max_happiness = max_happiness.max(local_max);
    }
    Some(max_happiness)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(330));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(286));
    }
}
