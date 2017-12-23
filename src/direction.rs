use position::Position;


#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl Direction {
    pub fn as_offset(&self) -> Position {
        use self::Direction::*;
        match *self {
            Down => Position::new(0, 1),
            Left => Position::new(-1, 0),
            Right => Position::new(1, 0),
            Up => Position::new(0, -1),
        }
    }

    pub fn is_vertical(&self) -> bool {
        use self::Direction::*;
        *self == Up || *self == Down
    }

}


#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    pub position: Position,
    pub direction: Direction,
}

impl Location {
    pub fn new(position: Position, direction: Direction) -> Self {
        Location{position, direction}
    }
}
