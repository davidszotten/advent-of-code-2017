use shared::AppResult;
use position::Position;


struct Map {
    pub map: Vec<Vec<char>>,
}

impl Map {
    pub fn new(map: Vec<Vec<char>>) -> Self {
        Map{map: map}
    }

    pub fn get(&self, position: Position) -> char {
        self.map[position.y as usize][position.x as usize]
    }
}


#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl Direction {
    fn as_offset(&self) -> Position {
        use self::Direction::*;
        match *self {
            Down => Position::new(0, 1),
            Left => Position::new(-1, 0),
            Right => Position::new(1, 0),
            Up => Position::new(0, -1),
        }
    }

    fn is_vertical(&self) -> bool {
        use self::Direction::*;
        *self == Up || *self == Down
    }

}

#[derive(Debug, PartialEq, Clone)]
struct Location {
    position: Position,
    direction: Direction,
}


fn parse(input: &str) -> Map {
    Map::new(input.split('\n').map(|line| line.chars().collect()).collect())
}

fn start(map: &Map) -> Location {
    use self::Direction::*;
    if let Some(col) = map.map[0].iter().position(|&c| c != ' ') {
        return Location { position: Position::new(col as i32, 0), direction: Down};
    }
    unreachable!("no start found");
}

fn step(map: &Map, start: Location) -> Location {
    use self::Direction::*;
    let Location{position, direction} = start;
    if map.get(position) == '+' {
        if direction.is_vertical() {
            if map.get(position + Position::new(-1, 0)) != ' ' {
                return Location {
                    position: Position::new(position.x - 1, position.y),
                    direction: Left,
                }
            }
            if map.get(position + Position::new(1, 0)) != ' ' {
                return Location {
                    position: Position::new(position.x + 1, position.y),
                    direction: Right,
                }
            }
            panic!("lost");
        }
        else {
            if map.get(position + Position::new(0, -1)) != ' ' {
                return Location {
                    position: position + Position::new(0, -1),
                    direction: Up,
                }
            }
            if map.get(position + Position::new(0, 1)) != ' ' {
                return Location {
                    position: position + Position::new(0, 1),
                    direction: Down,
                }
            }
            panic!("lost");
        }

    }
    return Location {
        position: position + direction.as_offset(),
        direction: direction,
    }
}

fn walk(map: &Map, start: Location) -> (Vec<char>, u32) {
    let mut letters = vec![];
    let mut pos = start;
    let mut steps = 0;
    loop {
        // println!("{:?}", pos);
        let current = map.get(pos.position);
        // println!("current {:?}", current);
        if current >= 'A' && current <= 'Z' {
            letters.push(current);
        }
        if current == ' ' {
            break;
        }

        pos = step(&map, pos);
        steps += 1;
    }

    (letters, steps)
}

pub fn part1(input: &str) -> AppResult<u32> {
    let map = parse(input);
    let letters = walk(&map, start(&map)).0;
    let s: String = letters.iter().collect();
    println!("{}", s);
    Ok(0)
}


pub fn part2(input: &str) -> AppResult<u32> {
    let map = parse(input);
    let steps = walk(&map, start(&map)).1;
    Ok(steps)
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = concat!(
"     |          \n",
"     |  +--+    \n",
"     A  |  C    \n",
" F---|----E|--+ \n",
"     |  |  |  D \n",
"     +B-+  +--+ \n",
"                \n",
        );

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 0);
    }

    #[test]
    fn test_parse() {
        let map = parse(SAMPLE);

        assert_eq!(map.get(Position::new(5, 2)), 'A');
    }

    #[test]
    fn test_start() {
        assert_eq!(start(&parse(SAMPLE)), Location{
                position: Position::new(5, 0),
                direction: Direction::Down,
            });
    }

    #[test]
    fn test_walk() {
        let map = parse(SAMPLE);
        assert_eq!(
            walk(&map, start(&map)),
            (vec!['A', 'B', 'C', 'D', 'E', 'F'], 38)
        )
    }
}
