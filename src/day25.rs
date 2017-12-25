use std::collections::HashSet;
use shared::AppResult;

enum State {
    A,
    B,
}

struct Machine {
    pos: i32,
    ones: HashSet<i32>,
    state: State,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            pos: 0,
            ones: HashSet::new(),
            state: State::A,
        }
    }

    fn is_zero(&self) -> bool {
        !self.ones.contains(&self.pos)
    }

    fn write(&mut self, value: usize) {
        match value {
            0 => {self.ones.remove(&self.pos);},
            1 => {self.ones.insert(self.pos);},
            _ => {panic!("invalid write value");},
        }
    }

    fn move_right(&mut self) {
        self.pos += 1;
    }

    fn move_left(&mut self) {
        self.pos -= 1;
    }

    pub fn step(&mut self) {
        match self.state {
            State::A => {
                if self.is_zero() {
                    self.write(1);
                    self.move_right();
                }
                else {
                    self.write(0);
                    self.move_left();
                }
                self.state = State::B;
            },
            State::B => {
                if self.is_zero() {
                    self.write(1);
                    self.move_left();
                }
                else {
                    self.move_right();
                }
                self.state = State::A;
            },
        }
    }
}

pub fn part1(_input: &str) -> AppResult<u32> {
    let mut machine = Machine::new();
    // for _ in 0..12386363 {
    for _ in 0..6 {
        machine.step();
    }
    Ok(machine.ones.len() as u32)
}


pub fn part2(_input: &str) -> AppResult<u32> {
    Ok(0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("").unwrap(), 0);
    }
}
