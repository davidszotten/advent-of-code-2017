use nom::alpha;
use parsers::integer;


pub type Reg = char;


#[derive(Debug, PartialEq, Clone)]
pub enum Target {
    Value(i64),
    Register(Reg),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Snd(Target),
    Set(Reg, Target),
    Add(Reg, Target),
    Sub(Reg, Target),
    Mul(Reg, Target),
    Mod(Reg, Target),
    Rcv(Reg),
    Jgz(Target, Target),
    Jnz(Target, Target),
}

named!(pub parse_op <Op>,
    alt!(
        parse_snd |
        parse_set |
        parse_add |
        parse_sub |
        parse_mul |
        parse_mod |
        parse_rcv |
        parse_jgz |
        parse_jnz
    )
);

named!(parse_target <Target>,
    alt!(
        alpha => {|r: &[u8]| Target::Register(r[0] as Reg)} |
        integer => {|n| Target::Value(n as i64) }
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

named!(parse_sub <Op>,
    map!(preceded!(tag!("sub "), parse_target_value), |tv| Op::Sub(tv.0, tv.1))
);

named!(parse_mul <Op>,
    map!(preceded!(tag!("mul "), parse_target_value), |tv| Op::Mul(tv.0, tv.1))
);

named!(parse_mod <Op>,
    map!(preceded!(tag!("mod "), parse_target_value), |tv| Op::Mod(tv.0, tv.1))
);

named!(parse_rcv <Op>,
    map!(preceded!(tag!("rcv "), alpha), |r: &[u8]| Op::Rcv(r[0] as Reg))
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

named!(parse_jnz <Op>,
    do_parse!(
        tag!("jnz ") >>
        x: parse_target >>
        tag!(" ") >>
        y: parse_target >>
        (Op::Jnz(x, y))
    )
);
