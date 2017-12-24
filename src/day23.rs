use nom::IResult;
use shared::AppResult;
use tablet::{Op, Reg, Target, parse_op};

struct Program {
    registers: [i64; 8],
    ops: Vec<Op>,
    pc: i64,
}

impl Program {
    pub fn new(
        ops: &Vec<Op>,
        registers: [i64; 8]
    ) -> Self {
        Program {
            pc: 0,
            ops: ops.iter().cloned().collect(),
            registers: registers,
        }
    }

    fn _index(&self, reg: Reg) -> usize {
        match reg {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => panic!("invalid register"),
        }
    }

    fn resolve(&self, target: &Target) -> i64 {
        use self::Target::*;
        match target {
            Value(ref i) => *i,
            Register(r) => {
                let index = self._index(*r);
                self.registers[index]
            },
        }
    }

    fn set(&mut self, target: Reg, value: i64) {
        self.registers[self._index(target)] = value;
    }

    pub fn run(&mut self) -> u32 {
        use self::Op::*;
        use self::Target::*;

        let mut mul_count = 0;

        while self.pc < self.ops.len() as i64 && self.pc >= 0 {
            // println!("{:?}", &self.ops[self.pc as usize]);
            let op = &self.ops[self.pc as usize].clone();
            match op {
                Set(reg, t) => {
                    let value = self.resolve(&t);
                    self.set(*reg, value);
                },
                Sub(x, t) => {
                    let xval = self.resolve(&Register(*x));
                    let tval = self.resolve(&t);
                    self.set(*x, xval - tval);
                },
                Mul(x, t) => {
                    mul_count += 1;
                    let xval = self.resolve(&Register(*x));
                    let tval = self.resolve(&t);
                    self.set(*x, xval * tval);
                },
                Jnz(x, t) => {
                    let xval = self.resolve(&x);
                    let tval = self.resolve(&t);
                    if xval != 0 {
                        self.pc = self.pc + tval - 1;
                    }
                },
                Snd(_) => {}, // not used
                Add(_, _) => {}, // not used
                Mod(_, _) => {}, // not used
                Rcv(_) => {}, // not used
                Jgz(_, _) => {}, // not used
            }
            self.pc += 1;
        }
    mul_count
    }
}

fn parse(input: &str) -> Vec<Op> {
    input.split('\n')
        .filter_map(|line| match parse_op(line.as_bytes()) {
            IResult::Done(_, p) => Some(p),
            e => panic!("parsing failed: {}: {:?}", line, e),
        })
        .collect()
}

pub fn part1(input: &str) -> AppResult<u32> {
    let ops = parse(input);
    // println!("{:?}", program);
    Ok(Program::new(&ops, 0).run() as u32)
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
