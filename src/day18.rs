use nom::{IResult, alpha};
use parsers::integer;
use shared::AppResult;
use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;


type Reg = char;


#[derive(Debug, PartialEq, Clone)]
enum Target {
    Value(i64),
    Register(Reg),
}

#[derive(Debug, PartialEq, Clone)]
enum Op {
    Snd(Target),
    Set(Reg, Target),
    Add(Reg, Target),
    Mul(Reg, Target),
    Mod(Reg, Target),
    Rcv(Reg),
    Jgz(Target, Target),
}

named!(parse_op <Op>,
    alt!(
        parse_snd |
        parse_set |
        parse_add |
        parse_mul |
        parse_mod |
        parse_rcv |
        parse_jgz
    )
);

named!(parse_target <Target>,
    alt!(
        alpha => {|r: &[u8]| Target::Register(r[0] as Reg)} |
        integer => {|n| Target::Value(n as i64) }
    )
);

named!(parse_target_value <(Reg, Target)>,
    do_parse!(
        x: map!(alpha, |r: &[u8]| r[0] as Reg) >>
        tag!(" ") >>
        y: parse_target >>
        ((x, y))
    )
);

named!(parse_snd <Op>,
    map!(preceded!(tag!("snd "), parse_target), |t| Op::Snd(t))
);

named!(parse_set <Op>,
    map!(preceded!(tag!("set "), parse_target_value), |tv| Op::Set(tv.0, tv.1))
);

named!(parse_add <Op>,
    map!(preceded!(tag!("add "), parse_target_value), |tv| Op::Add(tv.0, tv.1))
);

named!(parse_mul <Op>,
    map!(preceded!(tag!("mul "), parse_target_value), |tv| Op::Mul(tv.0, tv.1))
);

named!(parse_mod <Op>,
    map!(preceded!(tag!("mod "), parse_target_value), |tv| Op::Mod(tv.0, tv.1))
);

named!(parse_rcv <Op>,
    map!(preceded!(tag!("rcv "), alpha), |r: &[u8]| Op::Rcv(r[0] as Reg))
);

named!(parse_jgz <Op>,
    do_parse!(
        tag!("jgz ") >>
        x: parse_target >>
        tag!(" ") >>
        y: parse_target >>
        (Op::Jgz(x, y))
    )
);

struct Program {
    _pid: i64,
    registers: HashMap<Reg, i64>,
    ops: Vec<Op>,
    pc: i64,
    input: Receiver<Option<i64>>,
    output: Sender<Option<i64>>,
}

impl Program {
    pub fn new(
        ops: &Vec<Op>,
        program_id: i64,
        input: Receiver<Option<i64>>,
        output: Sender<Option<i64>>,
    ) -> Self {
        Program {
            _pid: program_id,
            pc: 0,
            ops: ops.iter().cloned().collect(),
            registers: [('p', program_id)].iter().cloned().collect(),
            input: input,
            output: output,
        }
    }

    fn resolve(&self, target: &Target) -> i64 {
        use self::Target::*;
        match target {
            Value(ref i) => *i,
            Register(r) => *(self.registers.get(&r).unwrap_or(&0)),
        }
    }

    pub fn run(&mut self) -> u32 {
        use self::Op::*;
        use self::Target::*;

        let mut snd_count = 0;

        while self.pc < self.ops.len() as i64 && self.pc >= 0 {
            // println!("{:?}, {:?}", &self._pid, &self.ops[self.pc as usize]);
            match &self.ops[self.pc as usize] {
                Snd(ref t) => {
                    let value = self.resolve(t);
                    snd_count += 1;
                    match self.output.send(Some(value)) {
                        Ok(_) => {},
                        Err(e) => {println!("failed to send: {:?}", e);},
                    };
                },
                Set(reg, t) => {
                    let value = self.resolve(&t);
                    self.registers.insert(*reg, value);
                },
                Add(x, t) => {
                    let xval = self.resolve(&Register(*x));
                    let tval = self.resolve(&t);
                    self.registers.insert(*x, xval + tval);
                },
                Mul(x, t) => {
                    let xval = self.resolve(&Register(*x));
                    let tval = self.resolve(&t);
                    self.registers.insert(*x, xval * tval);
                },
                Mod(x, t) => {
                    let xval = self.resolve(&Register(*x));
                    let tval = self.resolve(&t);
                    self.registers.insert(*x, xval % tval);
                },
                Rcv(t) => {
                    match self.input.recv_timeout(Duration::from_millis(5000)) {
                        Ok(Some(value)) => {self.registers.insert(*t, value);},
                        Ok(_) => panic!("shouldn't recv None atm"),
                        Err(mpsc::RecvTimeoutError::Timeout) => {
                            return snd_count;
                        }
                        Err(_) => panic!("other recv error"),
                    }
                },
                Jgz(x, t) => {
                    let xval = self.resolve(&x);
                    let tval = self.resolve(&t);
                    if xval > 0 {
                        self.pc = self.pc + tval - 1;
                    }
                },
            }
            self.pc += 1;
        }
    snd_count
    }
}

fn run(program: &[Op]) -> i64 {
    use self::Op::*;
    use self::Target::*;

    let mut pc: i64 = 0;
    let mut latest_snd = 0;
    let mut registers = HashMap::new();

    while pc < program.len() as i64 && pc >= 0 {
        println!("{}, {:?}", latest_snd, &registers);
        println!("{:?}", &program[pc as usize]);
        match &program[pc as usize] {
            Snd(ref t) => {latest_snd = resolve(t, &registers);},
            Set(reg, t) => {
                let value = resolve(&t, &registers);
                registers.insert(*reg, value);
            },
            Add(x, t) => {
                let xval = resolve(&Register(*x), &registers);
                let tval = resolve(&t, &registers);
                registers.insert(*x, xval + tval);
            },
            Mul(x, t) => {
                let xval = resolve(&Register(*x), &registers);
                let tval = resolve(&t, &registers);
                registers.insert(*x, xval * tval);
            },
            Mod(x, t) => {
                let xval = resolve(&Register(*x), &registers);
                let tval = resolve(&t, &registers);
                registers.insert(*x, xval % tval);
            },
            Rcv(t) => {
                let tval = resolve(&Register(*t), &registers);
                if tval != 0 {
                    return latest_snd;
                }
            },
            Jgz(x, t) => {
                let xval = resolve(x, &registers);
                let tval = resolve(&t, &registers);
                if xval > 0 {
                    pc = pc + tval - 1;
                }
            },
        }
        pc += 1;
    }
    panic!("program ended")
}

fn resolve(target: &Target, registers: &HashMap<Reg, i64>) -> i64 {
    use self::Target::*;
    match target {
        Value(ref i) => *i,
        Register(r) => *(registers.get(&r).unwrap_or(&0)),
    }
}

fn parse(input: &str) -> Vec<Op> {
    input.split('\n')
        .filter_map(|line| match parse_op(line.as_bytes()) {
            IResult::Done(_, p) => Some(p),
            _ => panic!("parsing failed"),
        })
        .collect()
}

pub fn part1(input: &str) -> AppResult<u32> {
    let program = parse(input);
    // println!("{:?}", program);
    Ok(run(&program) as u32)
}


pub fn part2(input: &str) -> AppResult<u32> {
    let instructions = parse(input);
    let (p0t, p1r) = mpsc::channel();
    let (p1t, p0r) = mpsc::channel();
    let mut p0 = Program::new(
        &instructions,
        0,
        p0r,
        p0t,
    );
    let mut p1 = Program::new(
        &instructions,
        1,
        p1r,
        p1t,
    );
    let run1 = thread::spawn(move || p1.run());
    let run0 = thread::spawn(move || p0.run());
    run0.join().expect("t0 failed");
    let result = run1.join().expect("t1 failed");
    Ok(result)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2").unwrap(), 4);
    }

    #[test]
    fn test_parse_snd() {
        assert_eq!(
            parse_snd(&b"snd 5"[..]),
            IResult::Done(&b""[..], Op::Snd(Target::Value(5)))
        );
        assert_eq!(
            parse_snd(&b"snd a"[..]),
            IResult::Done(&b""[..], Op::Snd(Target::Register('a')))
        );
    }

    #[test]
    fn test_parse_set() {
        assert_eq!(
            parse_set(&b"set a 5"[..]),
            IResult::Done(&b""[..], Op::Set('a', Target::Value(5)))
        );
        assert_eq!(
            parse_set(&b"set a b"[..]),
            IResult::Done(&b""[..], Op::Set('a', Target::Register('b')))
        );
    }

    #[test]
    fn test_parser() {
        use self::Op::*;
        use self::Target::*;

        let input = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";
        let parsed: Vec<_> = input.split('\n')
            .filter_map(|line| match parse_op(line.as_bytes()) {
                IResult::Done(_, p) => Some(p),
                _ => panic!("parsing failed"),
            })
            .collect();
        assert_eq!(parsed, vec![
            Set('a', Value(1)),
            Add('a', Value(2)),
            Mul('a', Register('a')),
            Mod('a', Value(5)),
            Snd(Register('a')),
            Set('a', Value(0)),
            Rcv('a'),
            Jgz(Register('a'), Value(-1)),
            Set('a', Value(1)),
            Jgz(Register('a'), Value(-2)),
        ]);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d").unwrap(), 3);
    }


}
