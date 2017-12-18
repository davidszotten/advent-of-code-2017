use shared::AppResult;
use parsers::positive_integer;
use nom::alpha;


#[derive(Debug, PartialEq)]
enum Target {
    Value(u32),
    Register(char),
}

#[derive(Debug, PartialEq)]
enum Op {
    Snd(Target),
    Set(char, Target),
}

named!(parse_target <Target>,
    alt!(
        alpha => {|r: &[u8]| Target::Register(r[0] as char)} |
        positive_integer => {|n| Target::Value(n) }
    )
);

named!(parse_snd <Op>,
    map!(preceded!(tag!("snd "), parse_target), |t| Op::Snd(t))
);

named!(parse_set <Op>,
    do_parse!(
        tag!("set ") >>
        x: map!(alpha, |r: &[u8]| r[0] as char) >>
        tag!(" ") >>
        y: parse_target >>
        (Op::Set(x, y))
    )
);

pub fn part1(_input: &str) -> AppResult<u32> {
    Ok(0)
}


pub fn part2(_input: &str) -> AppResult<u32> {
    Ok(0)
}


#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;

    #[test]
    fn test_part1() {
        assert_eq!(part1("").unwrap(), 0);
    }

    #[test]
    fn test_parse_snd() {
        assert_eq!(
            parse_snd(&b"snd 5"[..]),
            IResult::Done(&b""[..], Op::Snd(Target::Value(5)))
        );
        assert_eq!(
            parse_snd(&b"snd a"[..]),
            IResult::Done(&b""[..], Op::Snd(Target::Register('a')))
        );
    }

    #[test]
    fn test_parse_set() {
        assert_eq!(
            parse_set(&b"set a 5"[..]),
            IResult::Done(&b""[..], Op::Set('a', Target::Value(5)))
        );
        assert_eq!(
            parse_set(&b"set a b"[..]),
            IResult::Done(&b""[..], Op::Set('a', Target::Register('b')))
        );
    }
}
