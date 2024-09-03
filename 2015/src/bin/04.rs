advent_of_code::solution!(4);

fn hash_starts_with_n_zeros(input: String, n: usize) -> bool {
    let zeros = "0".repeat(n);
    let digest = format!("{:x}", md5::compute(input));
    digest.starts_with(&zeros)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut n: u32 = 1;
    loop {
        let digest = format!("{}{}", input.trim(), n);
        if hash_starts_with_n_zeros(digest, 5) {
            break Some(n);
        }
        n = n.checked_add(1)?;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut n: u32 = 1;
    loop {
        let digest = format!("{}{}", input.trim(), n);
        if hash_starts_with_n_zeros(digest, 6) {
            break Some(n);
        }
        n = n.checked_add(1)?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(609043));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6742839));
    }
}
