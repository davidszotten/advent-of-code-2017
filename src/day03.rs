use shared::AppResult;

pub fn part1(input: String) -> AppResult<u32> {
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

fn as_offset(direction: &Direction) -> (i32, i32) {
    match *direction {
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
        Direction::Up => (0, 1),
        Direction::Down => (0, -1),
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
    position: (i32, i32),
}

impl Spiral {
    pub fn new() -> Self {
        Spiral {
            increment: true,
            steps: 0,
            length: 0,
            direction: Direction::Down,
            position: (0, 1),
        }
    }
}

impl Iterator for Spiral {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.position = (
            self.position.0 + as_offset(&self.direction).0,
            self.position.1 + as_offset(&self.direction).1
        );

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
        println!("{:?}", self);
        Some(self.position)
    }
}


fn find_coors(location: i32) -> (i32, i32) {
    return Spiral::new().nth((location -1) as usize).expect("spiral should always have more");
}

fn distance(position: (i32, i32)) -> u32 {
    (position.0.abs() + position.1.abs()) as u32
}


pub fn part2(_input: String) -> AppResult<u32> {
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
        (0, 0),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (2, -1),
        (2, 0),
        (2, 1),
        (2, 2),
        (1, 2),
    ])
    }

//     #[test]
//     fn test_part2() {
//         let input = "5 9 2 8
// 9 4 7 3
// 3 8 6 5";
//     assert_eq!(part2(input.into()).expect("failed"), 9);
//     }
}
