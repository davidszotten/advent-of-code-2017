use shared::AppResult;

pub fn part1(input: String) -> AppResult<u32> {
    let position: i32 = input.parse()?;
    let result = find_coors(position);
    // println!("{:?}", result);
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


fn find_coors(location: i32) -> (i32, i32) {
    let location = location - 1;
    let mut total_steps = 0;
    let mut steps = 1;
    let mut increment = false;
    let mut direction = Direction::Right;
    let mut position = (0, 0);
    loop {
        if  location == total_steps {
            break position;
        }
        // println!("loc: {}, tot: {}, steps: {}", location, total_steps, steps);
        if  location < total_steps + steps {
            // println!("loc: {} < tot + steps: {}", location, total_steps+ steps);
            // println!("loc: {}, tot: {}", location, total_steps);
            steps = location - total_steps;
            // println!("steps: {}", steps);
        }
        // println!("{:?} {}", direction, steps);
        position = (
            position.0 + steps * as_offset(&direction).0,
            position.1 + steps * as_offset(&direction).1
        );
        total_steps += steps;

        // next
        direction = next(&direction);
        if increment {
            steps += 1;
        }
        increment = !increment;
    }
}

fn distance(position: (i32, i32)) -> u32 {
    (position.0.abs() + position.1.abs()) as u32
}


pub fn part2(input: String) -> AppResult<u32> {
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

//     #[test]
//     fn test_part2() {
//         let input = "5 9 2 8
// 9 4 7 3
// 3 8 6 5";
//     assert_eq!(part2(input.into()).expect("failed"), 9);
//     }
}
