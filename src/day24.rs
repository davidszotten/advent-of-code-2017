use shared::AppResult;

fn parse(input: &str) -> Vec<(u32, u32)> {
    let mut result = vec![];
    for line in input.split('\n') {
        let mut parts = line.split('/');
        let a = parts.next().expect("parsing failed").parse().expect("p fail");
        let b = parts.next().expect("parsing failed").parse().expect("p fail");
        result.push((a, b));
    }
    result
}

fn build(parts: Vec<(u32, u32)>, start: u32) -> u32 {
    let options = parts
        .iter()
        .enumerate()
        .filter(|(_, &(x, y))| *x == start || *y == start);
    let mut strongest = 0;
    for (index, &(x, y)) in options {
        let mut next_parts = parts.clone();
        next_parts.remove(index);
        let next_start = if x == start {y} else {x};
        let strength = x + y + build(next_parts, next_start);
        if strength > strongest {
            strongest = strength;
        }
    }
    strongest
}

fn build2(parts: Vec<(u32, u32)>, start: u32) -> (u32, u32) {
    let options = parts
        .iter()
        .enumerate()
        .filter(|(_, &(x, y))| *x == start || *y == start);
    let mut longest_strongest = (0, 0);
    for (index, &(x, y)) in options {
        let mut next_parts = parts.clone();
        next_parts.remove(index);
        let next_start = if x == start {y} else {x};
        let next = build2(next_parts, next_start);
        let length_strength = (next.0 + 1, next.1 +x + y);
        if length_strength > longest_strongest {
            longest_strongest = length_strength;
        }
    }
    longest_strongest
}

pub fn part1(input: &str) -> AppResult<u32> {
    Ok(build(parse(input), 0))
}


pub fn part2(input: &str) -> AppResult<u32> {
    Ok(build2(parse(input), 0).1)
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

    #[test]
    fn test_parsing() {
        assert_eq!(parse(SAMPLE), vec![(0,2), (2,2), (2,3), (3,4), (3,5), (0,1), (10,1), (9,10)]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), 19);
    }
}
