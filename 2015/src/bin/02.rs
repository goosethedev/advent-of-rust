advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.trim().split('\n') {
        let measures: Vec<u32> = line.split('x').map(|v| v.parse().unwrap()).collect();
        let prod: u32 = measures.iter().product();
        let couples_sum: u32 = measures.iter().fold(0, |acc, cur| acc + (prod / cur));
        let max: u32 = *measures.iter().max()?;
        total += (2 * couples_sum) + (prod / max);
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.trim().split('\n') {
        let measures: Vec<u32> = line.split('x').map(|v| v.parse().unwrap()).collect();
        let prod: u32 = measures.iter().product();
        let sum: u32 = measures.iter().sum();
        let max: u32 = *measures.iter().max()?;
        total += 2 * (sum - max) + prod;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(58));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
