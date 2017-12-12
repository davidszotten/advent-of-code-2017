use parsers::positive_integer;
use shared::AppResult;
use std::collections::{HashSet, VecDeque};
use nom::IResult;

#[derive(Debug, PartialEq)]
struct Pipe {
    from: u32,
    to: Vec<u32>,
}

named!(pipe <Pipe>,
    do_parse!(
        from: positive_integer >>
        tag!(" <-> ") >>
        to: separated_list!(
            complete!(tag!(", ")),
            positive_integer
        ) >>
        (Pipe{from, to})
    )
);

fn walk(pipes: &[Pipe], start: u32) -> HashSet<u32> {
    let mut queue: VecDeque<u32> = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back(start);

    while let Some(pipe_number) = queue.pop_front() {
        seen.insert(pipe_number);
        let pipe = pipes.get(pipe_number as usize).expect("invalid pipe");
        for connection in pipe.to.iter() {
            if seen.contains(connection) {continue};
            queue.push_back(*connection);
        }
    }
    seen
}

fn parse(input: &str) -> Vec<Pipe> {
    input
        .split('\n')
        .filter_map(|row| match pipe(row.as_bytes()) {
            IResult::Done(_, pipe) => Some(pipe),
            _ => None,
        })
        .collect::<Vec<Pipe>>()
}

pub fn part1(input: &str) -> AppResult<u32> {
    let pipes = parse(input);
    Ok(walk(&pipes, 0).len() as u32)
}


pub fn part2(input: &str) -> AppResult<u32> {
    let mut groups = 0;
    let pipes = parse(input);
    let mut seen: HashSet<u32> = HashSet::new();
    for pipe in pipes.iter() {
        if seen.contains(&pipe.from) {
            continue
        }
        groups += 1;
        let mut walked = walk(&pipes, pipe.from);
        for p in walked.drain() {
            seen.insert(p);
        }
    }
    Ok(groups)
}


#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;

    #[test]
    fn test_parse_pipe() {
        assert_eq!(
            pipe(&b"0 <-> 2"[..]),
            IResult::Done(&b""[..], Pipe{from: 0, to: vec![2]})
        );
        assert_eq!(
            pipe(&b"2 <-> 0, 3, 4"[..]),
            IResult::Done(&b""[..], Pipe{from: 2, to: vec![0, 3, 4]})
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("0 <-> 0").unwrap(), 1);
        assert_eq!(part1("0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5").unwrap(), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("0 <-> 0").unwrap(), 1);
        assert_eq!(part2("0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5").unwrap(), 2);
    }
}
