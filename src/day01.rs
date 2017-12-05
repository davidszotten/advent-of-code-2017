use shared::AppResult;

pub fn part1(input: String) -> AppResult<u32> {
    count_with_skip(input, 1)
}


pub fn part2(input: String) -> AppResult<u32> {
    let skip = input.chars().count() / 2;
    count_with_skip(input, skip)
}


fn count_with_skip(input: String, skip: usize) -> AppResult<u32> {
    let mut sum = 0;

    for (a, b) in input.chars().zip(input.chars().cycle().skip(skip)) {
        if a == b {
            sum += a.to_digit(10).ok_or(format_err!("Not a digit"))?;
        }
    }
    Ok(sum)
}
