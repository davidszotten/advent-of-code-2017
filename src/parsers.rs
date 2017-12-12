use std::str::{self, FromStr};
use nom::digit;

named!(number <i32>,
    map_res!(
        map_res!(digit, str::from_utf8),
        FromStr::from_str
    )
);


named!(pub integer <i32>,
    do_parse!(
        negative: opt!(complete!(tag!("-"))) >>
        number: number >>
        (match negative {
            None => number,
            Some(_) => -number,
        })
    )
);


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
}
