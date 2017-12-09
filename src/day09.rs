use shared::AppResult;
// use itertools::Itertools;

pub fn part1(_input: &str) -> AppResult<u32> {
    Ok(0)
}


pub fn part2(_input: &str) -> AppResult<u32> {
    Ok(0)
}


// fn parse_group(input: &str, score: u32) -> AppResult<u32> {
//     let mut stream = input.chars();
//     assert!(stream.next().expect("empty input") == '{');
//     parse_subgroup(stream)
// }

fn parse_group(input: &mut impl Iterator<Item=char>, score: u32) -> AppResult<u32> {
    // let (mut input, tmp) = input.tee();
    // println!("{:?}", tmp.collect::<String>());
    println!("starting with {}", score);
    let mut count = score;
    let mut in_garbage = false;
    let mut cancel_next = false;
    if score == 1 {
        assert!(input.next().expect("empty input") == '{');
    }
    let mut steps = 0;
    loop {
        steps += 1;
        if steps > 100 {bail!("max steps exceeded")}
        let next = input.next();
        println!("{}, {}, {:?}", score, count, next);
        if in_garbage {
            if cancel_next {
                cancel_next = false;
            } else {
                match next {
                    Some('>') => {in_garbage = false},
                    Some('!') => {cancel_next = true},
                    Some(other) => {println!("found other in garbage: {:?}", other)},
                    None => bail!("unclosed garbage"),
                }
            }
        } else {
            match next {
                Some('{') => {
                        let sub = parse_group(input, score + 1)?;
                        println!("count: {}, sub: {}", count, sub);
                        count += sub;
                    },
                Some('}') => return Ok(count),
                Some('<') => {in_garbage = true},
                other => {println!("found other: {:?}", other)},
            };
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_group() {
        println!("");
        // assert_eq!(parse_group(&mut"{}".chars(), 1).unwrap(), 1);
        // assert_eq!(parse_group(&mut"{{}}".chars(), 1).unwrap(), 3);
        // assert_eq!(parse_group(&mut"{{{}}}".chars(), 1).unwrap(), 6);
        // assert_eq!(parse_group(&mut"{{},{}}".chars(), 1).unwrap(), 5);
        // assert_eq!(parse_group(&mut"{{{},{},{{}}}}".chars(), 1).unwrap(), 16);
        // assert_eq!(parse_group(&mut"{<a>,<a>,<a>,<a>}".chars(), 1).unwrap(), 1);
        // assert_eq!(parse_group(&mut"{{<ab>},{<ab>},{<ab>},{<ab>}}".chars(), 1).unwrap(), 9);
        // assert_eq!(parse_group(&mut"{{<!!>},{<!!>},{<!!>},{<!!>}}".chars(), 1).unwrap(), 9);
        assert_eq!(parse_group(&mut"{{<a!>},{<a!>},{<a!>},{<ab>}}".chars(), 1).unwrap(), 3);
    }
}
