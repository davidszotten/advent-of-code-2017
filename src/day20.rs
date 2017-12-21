use nom::IResult;
use parsers::integer;
use shared::AppResult;
use std::collections::HashMap;
use std::ops;


named!(parse_coor <Coor>,
    do_parse!(
        tag!("<") >>
        x: integer >>
        tag!(",") >>
        y: integer >>
        tag!(",") >>
        z: integer >>
        tag!(">") >>
        (Coor::new(x, y, z))
    )
);

named!(parse <Particle>,
    do_parse!(
        tag!("p=") >>
        p: parse_coor >>
        tag!(", v=") >>
        v: parse_coor >>
        tag!(", a=") >>
        a: parse_coor >>
        (Particle::new(p, v, a))
    )
);


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Particle {
    p: Coor,
    v: Coor,
    a: Coor,
}

impl Particle {
    fn new(p: Coor, v: Coor, a: Coor) -> Self {
        Particle {p, v, a}
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coor {
    x: i32,
    y: i32,
    z: i32,
}

impl Coor {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Coor{x, y, z}
    }

    fn checked_add(&self, other: Coor) -> Option<Self> {
        if let Some(x) = self.x.checked_add(other.x) {
            if let Some(y) = self.y.checked_add(other.y) {
                if let Some(z) = self.z.checked_add(other.z) {
                    return Some(Coor::new(x,y,z))
                }
            }
        }
        None
    }
}

impl ops::AddAssign for Coor {
    fn add_assign(&mut self, rhs: Coor) {
        *self = Coor::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        );
    }
}

impl ops::Add for Coor {
    type Output = Coor;
    fn add(self, rhs: Coor) -> Self::Output {
        Coor::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}




pub fn part1(input: &str) -> AppResult<u32> {
    let rows = input.split('\n')
        .filter_map(|row| match parse(row.as_bytes()) {
            IResult::Done(_, entry) => Some(entry),
            o => panic!("parsing failed {:?}", o),
        })
        .collect::<Vec<_>>();
    let mut largest_val = 0;
    let mut largest_pos = 0;

    for (index, mut particle) in rows.into_iter().enumerate() {
        // pretty hacky, rely on overflow
        let mut val = 0;
        loop {
            particle.v = if let Some(res) = particle.v.checked_add(particle.a) {res}
            else {break};
            particle.p = if let Some(res) = particle.p.checked_add(particle.v) {res}
            else {break};
            let partial = if let Some(res) = particle.p.x.abs().checked_add(particle.p.y) {res}
            else {break};
            let _pos = if let Some(res) = partial.checked_add(particle.p.z.abs()) {res}
            else {break};
            val += 1;
        }

        if val > largest_val {
            largest_val = val;
            largest_pos = index;
        }

    }
    Ok(largest_pos as u32)
}

#[derive(Debug, PartialEq)]
enum PState {
    Alive(Particle),
    Overflowed,
    Collided,
}


pub fn part2(input: &str) -> AppResult<u32> {
    use self::PState::*;

    let mut particles = input.split('\n')
        .filter_map(|row| match parse(row.as_bytes()) {
            IResult::Done(_, entry) => Some(entry),
            o => panic!("parsing failed {:?}", o),
        })
        .map(|p| Alive(p))
        .collect::<Vec<_>>();

    loop {
        let mut positions = HashMap::new();
        println!("{:?}", particles.iter().map(|p| match p {
            Alive(_) => Coor::new(1,0,0),
            Overflowed => Coor::new(0,1,0),
            Collided => Coor::new(0,0,1),
        }).fold(Coor::new(0,0,0), |acc, x| acc + x));
        println!("ov {}", particles.iter().filter(|p| match p {
            Overflowed => true,
            _ => false,
        }).count());
        if !particles.iter().any(|p| match p {
            Alive(_) => true,
            _ => false,
        }) {
            break;
        }
        // println!("{:?}", particles);
        for position in 0..particles.len() {
            if let Alive(mut particle) = particles[position] {

                match particle.v.checked_add(particle.a) {
                    Some(res) => {
                        particle.v = res;
                        match particle.p.checked_add(particle.v) {
                            Some(res) => {
                                particle.p = res;
                                particles[position] = Alive(particle);
                            },
                            None => particles[position] = Overflowed,
                        };
                    },
                    None => particles[position] = Overflowed,
                };

    // println!("{:?}", positions);

                let collided_pos = positions.entry(particle.p).or_insert(position);
                if *collided_pos != position {
                    // println!("{:?}, {}, {}", particle.p, collided_pos, position);
                    particles[*collided_pos] = Collided;
                    particles[position] = Collided;
                }
                // if let Some(collided) = positions.get(&particle.p) {
                //     particles[*collided] = Collided;
                //     particles[position] = Collided;
                // } else {
                //     positions.insert(particle.p, position);
                // }
            }
        }
    }

    Ok(particles.iter().filter(|p| match p {
        Alive(_) => panic!("still alive?"),
        Overflowed => true,
        Collided => false,
    }).count() as u32)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(&b"p=<-14,1556,-678>, v=<-17,-131,-2>, a=<2,2,8>"[..]),
            IResult::Done(&b""[..], Particle::new(
                Coor::new(-14,1556,-678), Coor::new(-17, -131, -2), Coor::new(2, 2, 8)))
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>").unwrap(), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>").unwrap(), 1);
    }
}
