use std::cmp;
use std::u32;
use shared::{AppResult};


pub fn part1(input: String) -> AppResult<u32> {
    let mut sum = 0;
    for row in input.split('\n') {
        let values = row.split_whitespace()
            .filter_map(|x| x.parse::<u32>().ok());
        let (min, max) = values.fold(
            (u32::max_value(), u32::min_value()),
            |(mn, mx), x| (cmp::min(mn, x), cmp::max(mx, x))
        );
        sum += max - min;
    }
    Ok(sum)
}


pub fn part2(input: String) -> AppResult<u32> {
    let mut sum = 0;
    for row in input.split('\n') {
        let values = row.split_whitespace()
            .filter_map(|x| x.parse::<u32>().ok())
            .collect::<Vec<_>>();
        sum += row_match(values);
    }
    Ok(sum)
}

fn row_match(values: Vec<u32>) -> u32 {
    for (i, x) in values.iter().enumerate() {
        for y in values.iter().skip(i + 1) {
            if x % y == 0 {
                return x / y;
            }
            if y % x == 0 {
                return y / x;
            }
        }
    }
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "5 1 9 5
7 5 3
2 4 6 8";
    assert_eq!(part1(input.into()).expect("failed"), 18);
    }

    #[test]
    fn test_part2() {
        let input = "5 9 2 8
9 4 7 3
3 8 6 5";
    assert_eq!(part2(input.into()).expect("failed"), 9);
    }
}
