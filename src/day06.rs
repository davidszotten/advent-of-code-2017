use std::collections::HashMap;
use shared::AppResult;

pub fn part1(input: &str) -> AppResult<u32> {
    Ok(calculate(input).0)
}


pub fn part2(input: &str) -> AppResult<u32> {
    Ok(calculate(input).1)
}


fn calculate(input: &str) -> (u32, u32) {
    let mut banks: Vec<i32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let len = banks.len();
    let mut seen = HashMap::new();
    let mut steps = 0;

    while !seen.contains_key(&banks) {
        let (maxpos, &blocks) = banks.iter()
            .enumerate()
            .max_by_key(|&(i, x)| (x, -(i as i32)))
            .expect("banks empty");
        // println!("{:?},    {}, {}", banks, blocks, maxpos);

        seen.insert(banks.clone(), steps);
        let mut blocks = blocks;

        banks[maxpos] = 0;
        let mut pos = maxpos;
        while blocks > 0 {
            pos = (pos + 1) % len;

            banks[pos] += 1;
            blocks -= 1;
        }
        steps += 1;
    }
    // println!("{:?}\n\n{:?}", banks, seen);
    let part1 = steps;
    let part2 = steps - *seen.get(&banks).expect("banks should be seen");
    (part1, part2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("0 2    7  0").expect("failed"), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("0 2    7  0").expect("failed"), 4);
    }
}
