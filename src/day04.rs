use std::collections::HashSet;
use shared::AppResult;

pub fn part1(input: &str) -> AppResult<u32> {
    let mut count = 0;
    'outer: for row in input.split('\n') {
        let mut seen = HashSet::new();
        for entry in row.split_whitespace() {
            if seen.contains(entry) {
                continue 'outer;
            }
            seen.insert(entry);
        }
        count += 1
    }
    Ok(count)
}


pub fn part2(input: &str) -> AppResult<u32> {
    let mut count = 0;
    'outer: for row in input.split('\n') {
        let mut seen = HashSet::new();
        for entry in row.split_whitespace() {
            let mut sorted = entry.chars().collect::<Vec<_>>();
            sorted.sort();
            if seen.contains(&sorted) {
                continue 'outer;
            }
            seen.insert(sorted);
        }
        count += 1
    }
    Ok(count)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("aa bb cc dd ee").expect("failed"), 1);
        assert_eq!(part1("aa bb cc dd ee aa").expect("failed"), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("abcde fghij").expect("failed"), 1);
        assert_eq!(part2("abcde xyz ecdab").expect("failed"), 0);
    }

}
