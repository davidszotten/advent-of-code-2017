use shared::AppResult;

pub fn part1(input: &str) -> AppResult<u32> {
    count_with_skip(input, 1)
}


pub fn part2(input: &str) -> AppResult<u32> {
    let skip = input.chars().count() / 2;
    count_with_skip(input, skip)
}


fn count_with_skip(input: &str, skip: usize) -> AppResult<u32> {
    let mut sum = 0;

    for (a, b) in input.chars().zip(input.chars().cycle().skip(skip)) {
        if a == b {
            sum += a.to_digit(10).ok_or(format_err!("Not a digit"))?;
        }
    }
    Ok(sum)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("1122").unwrap(), 3);
        assert_eq!(part1("1111").unwrap(), 4);
        assert_eq!(part1("1234").unwrap(), 0);
        assert_eq!(part1("91212129").unwrap(), 9);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("1212").unwrap(), 6);
        assert_eq!(part2("1221").unwrap(), 0);
        assert_eq!(part2("123425").unwrap(), 4);
        assert_eq!(part2("123123").unwrap(), 12);
        assert_eq!(part2("12131415").unwrap(), 4);
    }
}
