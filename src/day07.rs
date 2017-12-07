use std::collections::{HashSet, HashMap};
use std::str::{self, FromStr};
use nom::{IResult, alphanumeric, digit, space};

use shared::AppResult;

pub fn part1(input: &str) -> AppResult<u32> {
    println!("{:?}", bottom(input));
    Ok(0)
}


fn weight(program: &Program, program_map: &HashMap<String, Program>) -> u32 {
    let mut sum = program.weight;
    for child_name in program.children.clone() {
        let child = program_map.get(&child_name).expect("child not in map");
        sum += weight(&child, &program_map);
    }
    sum
}

fn balanced_children(
    program: &Program, program_map: &HashMap<String, Program>
) -> bool {

    if program.children.len() == 0 {
        return true;
    }
    let weights: HashSet<_> = program.children.iter()
        .map(|n| weight(
            &program_map.get(n).expect("child not in map"),
            &program_map
        ))
        .collect();
    weights.len() == 1
}


pub fn part2(input: &str) -> AppResult<u32> {
    let program_map: HashMap<_,_> = input.split('\n')
        .filter_map(|line| match program(line.as_bytes()) {
            IResult::Done(_, p) => Some(p),
            _ => None,
        })
        .map(|p| (p.name.clone(), p))
        .collect();

    for program in program_map.values() {
        let mut weights: Vec<_> = program.children.iter()
            .map(|n| weight(
                &program_map.get(n).expect("child not in map"),
                &program_map
            ))
            .collect();
        if weights.len() == 0 {
            continue;
        }

        weights.sort();

        let middle = *weights.get(weights.len() / 2).expect("no middle");

        for child_name in program.children.iter() {
            let child = program_map.get(child_name).expect("child not in map");
            let child_total_weight = weight(child, &program_map);
            if child_total_weight != middle && balanced_children(&child, &program_map) {
                return Ok(child.weight + middle - child_total_weight);
                // println!("### {} {:?} {}", child.name, balanced_children(&child, &program_map), child.weight + middle - child_total_weight);
            }
        }
    }
    Ok(0)
}


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
    do_parse!(
        names: separated_list!(
            complete!(tag!(", ")),
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



fn bottom(input: &str) -> AppResult<String> {
    let programs: Vec<Program> = input.split('\n')
        .filter_map(|line| match program(line.as_bytes()) {
            IResult::Done(_, p) => Some(p),
            _ => None,
        })
        .collect();
    let mut seen_left = HashSet::new();
    let mut seen_right = HashSet::new();
    for program in programs {
        seen_left.insert(program.name);
        for child in program.children {
            seen_right.insert(child);
        }
    }
    let bottom = seen_left.difference(&seen_right).next().expect("no bottom");
    Ok(bottom.clone())
}


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

    #[test]
    fn test_bottom() {
        assert_eq!(bottom("pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)").unwrap(), "tknk");
    }


    #[test]
    fn test_part2() {
        assert_eq!(part2("pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)").unwrap(), 60);
    }
}
