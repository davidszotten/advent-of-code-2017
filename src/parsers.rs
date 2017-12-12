use std::str::{self, FromStr};
use nom::digit;

named!(pub positive_integer <u32>,
    map_res!(
        map_res!(digit, str::from_utf8),
        FromStr::from_str
    )
);


named!(pub integer <i32>,
    do_parse!(
        negative: opt!(complete!(tag!("-"))) >>
        number: positive_integer >>
        (match negative {
            None => number as i32,
            Some(_) => -(number as i32),
        })
    )
);


#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;

    #[test]
    fn test_positive_integer() {
        assert_eq!(
            positive_integer(&b"42"[..]),
            IResult::Done(&b""[..], 42)
        );
    }

    #[test]
    fn test_integer_positive() {
        assert_eq!(
            integer(&b"42"[..]),
            IResult::Done(&b""[..], 42)
        );
    }

    #[test]
    fn test_integer_negative() {
        assert_eq!(
            integer(&b"-42"[..]),
            IResult::Done(&b""[..], -42)
        );
    }
}
