use shared::AppResult;

pub fn part1(input: &str) -> AppResult<u32> {
    let inc: usize = input.parse().expect("parsing failed");
    let mut buf = vec![0];
    let mut length = 1;
    let mut pos = 0;
    for step in 1..2018 {
    // for step in 1..10 {
        pos = (pos + inc) % length;
        buf.insert(pos + 1, step);
        length += 1;
        pos += 1; // just inserted so never needs to wrap
    }
    Ok(buf[(pos + 1) % length])
}


pub fn part2(input: &str) -> AppResult<u32> {
    let inc: usize = input.parse().expect("parsing failed");
    let mut after0 = 0;
    let mut length = 1;
    let mut pos = 0;
    for step in 1..50_000_000 {
        pos = (pos + inc) % length;
        if pos == 0 {
            after0 = step;
        }
        length += 1;
        pos += 1;
    }
    Ok(after0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test] #[ignore]
    fn test_part1() {
        assert_eq!(part1("3").unwrap(), 638);
    }
}
