use shared::AppResult;
use parsers::integer;
use nom::alpha;


type Reg = char;


#[derive(Debug, PartialEq)]
enum Target {
    Value(i32),
    Register(Reg),
}

#[derive(Debug, PartialEq)]
enum Op {
    Snd(Target),
    Set(Reg, Target),
    Add(Reg, Target),
    Mul(Reg, Target),
    Mod(Reg, Target),
    Rcv(Target),
    Jgz(Target, Target),
}

named!(parse_op <Op>,
    alt!(
        parse_snd |
        parse_set |
        parse_add |
        parse_mul |
        parse_mod |
        parse_rcv |
        parse_jgz
    )
);

named!(parse_target <Target>,
    alt!(
        alpha => {|r: &[u8]| Target::Register(r[0] as Reg)} |
        integer => {|n| Target::Value(n) }
    )
);

named!(parse_target_value <(Reg, Target)>,
    do_parse!(
        x: map!(alpha, |r: &[u8]| r[0] as Reg) >>
        tag!(" ") >>
        y: parse_target >>
        ((x, y))
    )
);

named!(parse_snd <Op>,
    map!(preceded!(tag!("snd "), parse_target), |t| Op::Snd(t))
);

named!(parse_set <Op>,
    map!(preceded!(tag!("set "), parse_target_value), |tv| Op::Set(tv.0, tv.1))
);

named!(parse_add <Op>,
    map!(preceded!(tag!("add "), parse_target_value), |tv| Op::Add(tv.0, tv.1))
);

named!(parse_mul <Op>,
    map!(preceded!(tag!("mul "), parse_target_value), |tv| Op::Mul(tv.0, tv.1))
);

named!(parse_mod <Op>,
    map!(preceded!(tag!("mod "), parse_target_value), |tv| Op::Mod(tv.0, tv.1))
);

named!(parse_rcv <Op>,
    map!(preceded!(tag!("rcv "), parse_target), |t| Op::Rcv(t))
);

named!(parse_jgz <Op>,
    do_parse!(
        tag!("jgz ") >>
        x: parse_target >>
        tag!(" ") >>
        y: parse_target >>
        (Op::Jgz(x, y))
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

    #[test]
    fn test_parser() {
        use self::Op::*;
        use self::Target::*;

        let input = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";
        let parsed: Vec<_> = input.split('\n')
            .filter_map(|line| match parse_op(line.as_bytes()) {
                IResult::Done(_, p) => Some(p),
                _ => None,
            })
            .collect();
        assert_eq!(parsed, vec![
            Set('a', Value(1)),
            Add('a', Value(2)),
            Mul('a', Register('a')),
            Mod('a', Value(5)),
            Snd(Register('a')),
            Set('a', Value(0)),
            Rcv(Register('a')),
            Jgz(Register('a'), Value(-1)),
            Set('a', Value(1)),
            Jgz(Register('a'), Value(-2)),
        ]);
    }
}
