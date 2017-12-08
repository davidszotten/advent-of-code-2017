use std::collections::{HashMap};
use std::cmp;
use std::str::{self, FromStr};
use shared::AppResult;
use nom::{IResult, alpha, digit, space};


#[derive(Debug, PartialEq)]
enum Operation {
    Decrement,
    Increment,
}

#[derive(Debug, PartialEq)]
enum ConditionOp {
    Eq,
    Gt,
    Gte,
    Ne,
    Lt,
    Lte,
}

#[derive(Debug, PartialEq)]
struct Program {
    register: String,
    operation: Operation,
    value: i32,
    condition_register: String,
    condition_op: ConditionOp,
    condition_value: i32,
}


named!(number <i32>,
    map_res!(
        map_res!(digit, str::from_utf8),
        FromStr::from_str
    )
);


named!(integer <i32>,
    do_parse!(
        negative: opt!(complete!(tag!("-"))) >>
        number: number >>
        (match negative {
            None => number,
            Some(_) => -number,
        })
    )
);


named!(program <Program>,
    do_parse!(
        register: map_res!(alpha, str::from_utf8) >>
        space >>
        op: alt!(
            tag!("inc") => {|_| Operation::Increment} |
            tag!("dec") => {|_| Operation::Decrement}
        ) >>
        space >>
        value: integer >>
        tag!(" if ") >>
        condition_register: map_res!(alpha, str::from_utf8) >>
        space >>
        condition_op: alt!(
            tag!(">=") => {|_| ConditionOp::Gte} |
            tag!(">") => {|_| ConditionOp::Gt} |
            tag!("<=") => {|_| ConditionOp::Lte} |
            tag!("<") => {|_| ConditionOp::Lt} |
            tag!("!=") => {|_| ConditionOp::Ne} |
            tag!("==") => {|_| ConditionOp::Eq}
        ) >>
        space >>
        condition_value: integer >>
        (Program {
            register: register.into(),
            operation: op,
            value: value,
            condition_register: condition_register.into(),
            condition_op: condition_op,
            condition_value: condition_value,
        })
    )
);

fn condition(left: i32, op: ConditionOp, right: i32) -> bool {
    use self::ConditionOp::*;
    match op {
        Eq => left == right,
        Gt => left > right,
        Gte => left >= right,
        Lt => left < right,
        Lte => left <= right,
        Ne => left != right,
    }
}

pub fn process(input: &str) -> AppResult<(u32, u32)> {
    let mut registers = HashMap::new();
    let mut maxduring = 0;

    for line in input.split('\n') {
        // println!("::{}::", line);
        let instruction = match program(line.as_bytes()) {
            IResult::Done(_, instruction) => instruction,
            err => bail!("Failed to parse instruction: {:?}", err),
        };
        let &condition_register_value = registers
            .get(&instruction.condition_register)
            .unwrap_or(&0);
        if condition(
                condition_register_value,
                instruction.condition_op,
                instruction.condition_value,
            ) {

            let mut register_value = registers
                .entry(instruction.register)
                .or_insert(0);
            match instruction.operation {
                Operation::Increment => {*register_value += instruction.value}
                Operation::Decrement => {*register_value -= instruction.value}
            }
        }
        maxduring = match registers.values().max() {
            Some(&n) => cmp::max(maxduring, n),
            None => maxduring,
        };
    }
    let maxend = match registers.values().max() {
        Some(&n) => n as u32,
        None => bail!("no registers"),
    };
    Ok((maxend, maxduring as u32))
}


pub fn part1(input: &str) -> AppResult<u32> {
    process(input).map(|(x, _)| x)
}


pub fn part2(input: &str) -> AppResult<u32> {
    process(input).map(|(_, y)| y)
}




#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;

    #[test]
    fn test_number() {
        assert_eq!(
            number(&b"42"[..]),
            IResult::Done(&b""[..], 42)
        );
    }

    #[test]
    fn test_positive_integer() {
        assert_eq!(
            integer(&b"42"[..]),
            IResult::Done(&b""[..], 42)
        );
    }

    #[test]
    fn test_negative_integer() {
        assert_eq!(
            integer(&b"-42"[..]),
            IResult::Done(&b""[..], -42)
        );
    }

    #[test]
    fn test_program() {
        assert_eq!(
            program(&b"b inc 5 if a > 1"[..]),
            IResult::Done(&b""[..], Program{
                    register: "b".into(),
                    operation: Operation::Increment,
                    value: 5,
                    condition_register: "a".into(),
                    condition_op: ConditionOp::Gt,
                    condition_value: 1,
                }
            )
        );
    }

    #[test]
    fn test_program2() {
        assert_eq!(
            program(&b"c dec -10 if a >= 1"[..]),
            IResult::Done(&b""[..], Program{
                    register: "c".into(),
                    operation: Operation::Decrement,
                    value: -10,
                    condition_register: "a".into(),
                    condition_op: ConditionOp::Gte,
                    condition_value: 1,
                }
            )
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10").unwrap(), 1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10").unwrap(), 10);
    }
}
