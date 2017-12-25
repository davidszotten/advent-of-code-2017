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
    pub fn step(&mut self) {
        match self.state {
            State::A => {
                if !self.ones.contains(&self.pos) {
                    self.ones.insert(self.pos);
                    self.pos += 1;
                }
                else {
                    self.ones.remove(&self.pos);
                    self.pos -= 1;
                }
                self.state = State::B;
            },
            State::B => {
                if !self.ones.contains(&self.pos) {
                    self.ones.insert(self.pos);
                    self.pos -= 1;
                }
                else {
                    self.pos += 1;
                }
                self.state = State::A;
            },
        }
    }
}

pub fn part1(_input: &str) -> AppResult<u32> {
    let mut machine = Machine::new();
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
