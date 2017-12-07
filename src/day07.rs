use std::str::{self, FromStr};
use nom::{IResult, alphanumeric, digit, space};

use shared::AppResult;

pub fn part1(input: String) -> AppResult<u32> {
    Ok(0)
}


pub fn part2(input: String) -> AppResult<u32> {
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

named!(program <Program>,
    do_parse!(
        name: map_res!(alphanumeric, str::from_utf8) >>
        space >>
        weight: number >>
        (Program{name: name.into(), weight: weight, children:vec![]})
    )
);



fn bottom(input: &str) -> AppResult<&str> {
    Ok("")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program() {
        assert_eq!(
            program(&b"pbga (66)"[..]),
            // program(&b"pbga"[..]),
            IResult::Done(&b""[..], Program{name: "pbga".into(), weight: 66, children: vec![]})
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
