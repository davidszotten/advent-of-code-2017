use nom::IResult;
use parsers::positive_integer;
use shared::AppResult;


named!(layer <(u32, u32)>,
    do_parse!(
        from: positive_integer >>
        tag!(": ") >>
        to: positive_integer >>
        ((from, to))
    )
);

fn parse(input: &str) -> Vec<(u32, u32)> {
    input.split('\n')
        .filter_map(|row| match layer(row.as_bytes()) {
            IResult::Done(_, tuple) => Some(tuple),
            _ => None,
        })
        .collect()
}


fn send(layers: &[(u32, u32)], delay: u32) -> u32 {
    layers
        .iter()
        .filter(|(depth, range)| (depth + delay) % (2 * range - 2) == 0)
        .map(|(depth, range)| depth * range)
        .sum()
}

fn hit(layers: &[(u32, u32)], delay: u32) -> bool {
    layers
        .iter()
        .any(|(depth, range)| (depth + delay) % (2 * range - 2) == 0)
}

pub fn part1(input: &str) -> AppResult<u32> {
    Ok(send(&parse(input), 0))
}


pub fn part2(input: &str) -> AppResult<u32> {
    let layers = parse(input);
    (0..)
    .skip_while(|&delay| hit(&layers, delay))
    .next()
    .map(|delay| delay)
    .ok_or(format_err!("unreachable"))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("0: 3
1: 2
4: 4"), vec![(0, 3), (1, 2), (4, 4)]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("0: 3
1: 2
4: 4
6: 4").unwrap(), 24);
    }

    #[test]
    fn test_hit() {
        assert!(hit(&[(0, 3)], 0));
        assert!(!hit(&[(1, 2)], 0));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("0: 3
1: 2
4: 4
6: 4").unwrap(), 10);
    }
}
