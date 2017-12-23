use std::collections::HashSet;
use shared::AppResult;
use position::Position;
use direction::{Direction, Location};

type Map = HashSet<Position>;

struct Map2 {
    weakened: HashSet<Position>,
    infected:  HashSet<Position>,
    flagged: HashSet<Position>,
}

fn parse(input: &str) -> HashSet<Position>{
    let mut map = HashSet::new();

    for (y, row) in input.split('\n').enumerate() {
        let offset = row.len() as i32 / 2;
        for (x, value) in row.chars().enumerate() {
            if value == '#' {
                map.insert(Position::new(x as i32 - offset, y as i32 - offset));
            }
        }
    }
    map
}

fn parse2(input: &str) -> Map2 {
    Map2 {
        weakened: HashSet::new(),
        infected: parse(input),
        flagged: HashSet::new(),
    }
}

fn turn_left(direction: Direction) -> Direction {
    use self::Direction::*;
    match direction {
        Left => Down,
        Down => Right,
        Right => Up,
        Up => Left,
    }
}

fn turn_right(direction: Direction) -> Direction {
    use self::Direction::*;
    match direction {
        Up => Right,
        Right => Down,
        Down => Left,
        Left => Up,
    }
}

fn walk2(mut map: Map2, steps: usize) -> usize {
    let start = Location::new(
        Position::new(0, 0),
        Direction::Up
    );
    let mut infections = 0;
    let mut current = start;

    for _ in 0..steps {
        if map.infected.contains(&current.position) {
            map.infected.remove(&current.position);
            map.flagged.insert(current.position);
            let next_direction = turn_right(current.direction);
            current = Location::new(
                current.position + next_direction.as_offset(),
                next_direction
            );
        }
        else if map.weakened.contains(&current.position) {
            map.weakened.remove(&current.position);
            infections += 1;
            map.infected.insert(current.position);
            let next_direction = current.direction;
            current = Location::new(
                current.position + next_direction.as_offset(),
                next_direction
            );
        }
        else if map.flagged.contains(&current.position) {
            map.flagged.remove(&current.position);
            let next_direction = turn_left(turn_left(current.direction));
            current = Location::new(
                current.position + next_direction.as_offset(),
                next_direction
            );
        }
        else {
            map.weakened.insert(current.position);
            let next_direction = turn_left(current.direction);
            current = Location::new(
                current.position + next_direction.as_offset(),
                next_direction
            );
        }
    }
    infections
}


fn walk(mut map: Map, steps: usize) -> usize {
    let start = Location::new(
        Position::new(0, 0),
        Direction::Up
    );
    let mut infections = 0;
    let mut current = start;

    for _ in 0..steps {
        if map.contains(&current.position) {
            map.remove(&current.position);
            let next_direction = turn_right(current.direction);
            current = Location::new(
                current.position + next_direction.as_offset(),
                next_direction
            );
        }
        else {
            infections += 1;
            map.insert(current.position);
            let next_direction = turn_left(current.direction);
            current = Location::new(
                current.position + next_direction.as_offset(),
                next_direction
            );
        }
    }
    infections
}

pub fn part1(input: &str) -> AppResult<u32> {
    Ok(walk(parse(input), 10000) as u32)
}


pub fn part2(input: &str) -> AppResult<u32> {
    Ok(walk2(parse2(input), 10_000_000) as u32)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("..#
#..
..."),
            [Position::new(1, -1), Position::new(-1, 0)].iter().cloned().collect()
    )
    }

    #[test]
    fn test_walk() {
        assert_eq!(walk(parse("..#
#..
..."), 70), 41);
        assert_eq!(walk(parse("..#
#..
..."), 10000), 5587);
    }

    #[test] #[ignore]
    fn test_walk2() {
        assert_eq!(walk2(parse2("..#
#..
..."), 100), 26);
        assert_eq!(walk2(parse2("..#
#..
..."), 10000000), 2511944);
    }
}
