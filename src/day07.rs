use std::str::{self, FromStr};
use nom::{alphanumeric, digit, space};

use shared::AppResult;

pub fn part1(_input: &str) -> AppResult<u32> {
    Ok(0)
}


pub fn part2(_input: &str) -> AppResult<u32> {
    Ok(0)
}


#[derive(Debug, PartialEq)]
struct Program {
    name: String,
    weight: u32,
    children: Vec<String>,
}


named!(number <u32>,
    map_res!(
        map_res!(
            delimited!(
                tag!("("),
                digit,
                tag!(")")
            ),
            str::from_utf8
        ),
        FromStr::from_str
    )
);

named!(children <Vec<String>>,
// named!(children <Vec<&str>>,
    do_parse!(
        // tag!(" -> ") >>
        names: separated_list!(
            complete!(tag!(", ")),
            // map_res!(complete!(alphanumeric), |s| str::from_utf8)
            do_parse!(
                name: map_res!(complete!(alphanumeric), str::from_utf8) >>
                (name.into())
                )
            ) >>
        (names)
    )
);


named!(program <Program>,
    do_parse!(
        name: map_res!(alphanumeric, str::from_utf8) >>
        space >>
        weight: number >>
        opt!(complete!(tag!(" -> "))) >>
        names: children >>
        (Program{name: name.into(), weight: weight, children: names})
    )
);



// fn bottom(input: &str) -> AppResult<&str> {
//     Ok("")
// }


#[cfg(test)]
mod tests {
    use nom::IResult;
    use super::*;

    #[test]
    fn test_children() {
        assert_eq!(
            children(&b"foo, bar"[..]),
            IResult::Done(&b""[..], vec!["foo".into(), "bar".into()]),
        );
        assert_eq!(
            children(&b""[..]),
            IResult::Done(&b""[..], vec![]),
        );
    }

    #[test]
    fn test_program_no_children() {
        assert_eq!(
            program(&b"pbga (66)"[..]),
            IResult::Done(&b""[..], Program{name: "pbga".into(), weight: 66, children: vec![]})
        );
    }

    #[test]
    fn test_program_children() {
        assert_eq!(
            program(&b"fwft (72) -> ktlj, cntj, xhth"[..]),
            IResult::Done(&b""[..], Program{name: "fwft".into(), weight: 72, children: vec!["ktlj".into(), "cntj".into(), "xhth".into()]})
        );
    }

//     #[test]
//     fn test_bottom() {
//         assert_eq!(bottom("pbga (66)
// xhth (57)
// ebii (61)
// havc (66)
// ktlj (57)
// fwft (72) -> ktlj, cntj, xhth
// qoyq (66)
// padx (45) -> pbga, havc, qoyq
// tknk (41) -> ugml, padx, fwft
// jptl (61)
// ugml (68) -> gyxo, ebii, jptl
// gyxo (61)
// cntj (57)").unwrap(), "tknk");
//     }
}
