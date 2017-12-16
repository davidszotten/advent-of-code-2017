use std::ops;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position{x, y}
    }

    pub fn neighbours(self) -> Vec<Position> {
        let mut result = vec![];
        for &x in [-1, 0, 1].iter() {
            for &y in [-1, 0, 1].iter() {
                result.push(self + Position::new(x, y));
            }
        }
        result
    }

    pub fn straight_neighbours(self) -> Vec<Position> {
        vec![
            self + Position::new(0, -1),
            self + Position::new(0, 1),
            self + Position::new(-1, 0),
            self + Position::new(1, 0),
        ]
    }
}

impl ops::Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign for Position {
    fn add_assign(&mut self, other: Position) {
        *self = Position {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

