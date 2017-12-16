use shared::AppResult;
use position::Position;
use std::collections::HashMap;

pub fn part1(input: &str) -> AppResult<u32> {
    let position: i32 = input.parse()?;
    let result = find_coors(position);
    Ok(distance(result))
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn as_offset(direction: &Direction) -> Position {
    match *direction {
        Direction::Left => Position::new(-1, 0),
        Direction::Right => Position::new(1, 0),
        Direction::Up => Position::new(0, 1),
        Direction::Down => Position::new(0, -1),
    }
}

fn next(direction: &Direction) -> Direction {
    match *direction {
        Direction::Left => Direction::Down,
        Direction::Right => Direction::Up,
        Direction::Up => Direction::Left,
        Direction::Down => Direction::Right,
    }
}


#[derive(Debug)]
struct Spiral {
    increment: bool,
    steps: i32,
    length: i32,
    direction: Direction,
    position: Position,
}

impl Spiral {
    pub fn new() -> Self {
        Spiral {
            increment: true,
            steps: 0,
            length: 0,
            direction: Direction::Down,
            position: Position::new(0, 1),
        }
    }
}

impl Iterator for Spiral {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        self.position += as_offset(&self.direction);

        if self.steps == self.length {
            self.steps = 1;
            self.direction = next(&self.direction);
            if self.increment {
                self.length += 1;
            }
            self.increment = !self.increment;
        } else {
            self.steps += 1;
        }
        // println!("{:?}", self);
        Some(self.position)
    }
}


fn find_coors(location: i32) -> Position {
    return Spiral::new().nth((location -1) as usize).expect("spiral should always have more");
}

fn distance(position: Position) -> u32 {
    (position.x.abs() + position.y.abs()) as u32
}


pub fn part2(input: &str) -> AppResult<u32> {
    let max: i32 = input.parse()?;
    let mut values = HashMap::new();
    values.insert(Position::new(0, 0), 1);
    for position in Spiral::new() {
        let value = position.neighbours().iter().fold(
            0,
            |acc, p| values.get(p).unwrap_or(&0) + acc
        );
        if value > max {
            return Ok(value as u32);
        }
        values.insert(position, value);
    }
    Ok(0)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(distance(find_coors(1)), 0);
        assert_eq!(distance(find_coors(12)), 3);
        assert_eq!(distance(find_coors(23)), 2);
        assert_eq!(distance(find_coors(1024)), 31);
    }

    #[test]
    fn test_spiral() {
        println!("");
        assert_eq!(Spiral::new().take(14).collect::<Vec<_>>(), vec![
        Position::new(0, 0),
        Position::new(1, 0),
        Position::new(1, 1),
        Position::new(0, 1),
        Position::new(-1, 1),
        Position::new(-1, 0),
        Position::new(-1, -1),
        Position::new(0, -1),
        Position::new(1, -1),
        Position::new(2, -1),
        Position::new(2, 0),
        Position::new(2, 1),
        Position::new(2, 2),
        Position::new(1, 2),
    ])
    }
}
