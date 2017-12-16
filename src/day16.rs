use shared::AppResult;
use parsers::positive_integer;
use nom::{IResult, alpha};


#[derive(Debug, PartialEq)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

named!(spin <Move>,
    do_parse!(
        tag!("s") >>
        n: positive_integer >>
        (Move::Spin(n as usize))
    )
);

named!(exchange <Move>,
    do_parse!(
        tag!("x") >>
        n: positive_integer >>
        tag!("/") >>
        m: positive_integer >>
        (Move::Exchange(n as usize, m as usize))
    )
);

named!(partner <Move>,
    do_parse!(
        tag!("p") >>
        x: alpha >>
        tag!("/") >>
        y: alpha >>
        (Move::Partner(x[0] as char, y[0] as char))
    )
);

named!(dance_move <Move>,
    alt!(spin | exchange | partner)
);


fn parse(input: &str) -> Vec<Move> {
    input.split(',')
        .filter_map(|row| match dance_move(row.as_bytes()) {
            IResult::Done(_, mv) => Some(mv),
            o => panic!("parsing failed {:?}", o),
        })
        .collect()
}

fn do_spin(mut line: Vec<char>, step: usize) -> Vec<char> {
    let len = line.len();
    let mut end = line.split_off(len - step);
    end.append(&mut line);
    end
}

fn do_exchange(mut line: Vec<char>, x: usize, y: usize) -> Vec<char> {
    line.swap(x, y);
    line
}

fn do_partner(line: Vec<char>, x: char, y: char) -> Vec<char> {
    let a = line.iter().position(|&c| c == x).expect("x not found");
    let b = line.iter().position(|&c| c == y).expect("y not found");
    do_exchange(line, a, b)
}


fn run(input: &str, programs: &str) -> String {
    let mut programs: Vec<_> = programs.chars().collect();
    // println!("{:?}", parse(input));
    for mv in parse(input) {
        match mv {
            Move::Spin(n) => {programs = do_spin(programs, n)},
            Move::Exchange(a, b) => {programs = do_exchange(programs, a, b)},
            Move::Partner(a, b) => {programs = do_partner(programs, a, b)},
        }
        // println!("{}", programs.iter().collect::<String>());
    }
    programs.iter().collect()
}

pub fn part1(input: &str) -> AppResult<u32> {
    println!("{}", run(input, "abcdefghijklmnop"));
    Ok(0)
}


pub fn part2(input: &str) -> AppResult<u32> {
    let mut program: String = "abcdefghijklmnop".into();
    // for _ in 0..1_000_000_000 {
    for _ in 0..1_000_00 {
        program = run(input, &program);
    }
    println!("{}", &program);
    Ok(0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spin() {
        assert_eq!(
            spin(&b"s10"[..]),
            IResult::Done(&b""[..], Move::Spin(10)
            )
        );
    }

    #[test]
    fn test_exchange() {
        assert_eq!(
            exchange(&b"x3/4"[..]),
            IResult::Done(&b""[..], Move::Exchange(3,4)
            )
        );
    }

    #[test]
    fn test_partner() {
        assert_eq!(
            partner(&b"pe/b"[..]),
            IResult::Done(&b""[..], Move::Partner('e', 'b')
            )
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("s1,x3/4,pe/b"), vec![Move::Spin(1), Move::Exchange(3, 4), Move::Partner('e', 'b')]
        );
    }

    #[test]
    fn test_do_spin() {
        let line = "abcde";
        assert_eq!(do_spin(line.chars().collect(), 1), "eabcd".chars().collect::<Vec<_>>());
    }

    #[test]
    fn test_do_exchange() {
        let line = "abcde";
        assert_eq!(do_exchange(line.chars().collect(), 1, 2), "acbde".chars().collect::<Vec<_>>());
    }

    #[test]
    fn test_do_partner() {
        let line = "abcde";
        assert_eq!(do_partner(line.chars().collect(), 'b', 'c'), "acbde".chars().collect::<Vec<_>>());
    }

    #[test]
    fn test_run() {
        assert_eq!(
            run("s1,x3/4,pe/b", "abcde"), "baedc"
        );
    }

}
