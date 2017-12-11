use std::cmp::max;
use std::ops;
use std::str::FromStr;
use failure::Error;
use shared::AppResult;

/*

        \  (0,1,-1) /
(-1,1,0) +---------+ (1,0,-1)
        /           \
   ----+             +----
        \           /
(-1,0,1) +---------+ (1,-1,0)
        / (0,-1,1)  \

*/


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coor {
    x: i32,
    y: i32,
    z: i32,
}

impl Coor {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Coor{x, y, z}
    }

    fn zero() -> Self {
        Coor::new(0, 0, 0)
    }

    fn distance(&self) -> u32 {
        max(max(self.x.abs(), self.y.abs()), self.z.abs()) as u32
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

impl FromStr for Coor {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "n" => Coor::new(0, 1, -1),
            "ne" => Coor::new(1, 0, -1),
            "nw" => Coor::new(-1, 1, 0),
            "s" => Coor::new(0, -1, 1),
            "se" => Coor::new(1, -1, 0),
            "sw" => Coor::new(-1, 0, 1),
            _ => bail!("invalid direction"),
        })
    }
}


pub fn part1(input: &str) -> AppResult<u32> {
    Ok(input
        .split(',')
        .filter_map(|c| c.parse::<Coor>().ok())
        .fold(Coor::zero(), |acc, c| acc + c)
        .distance()
    )
}


pub fn part2(input: &str) -> AppResult<u32> {
    let mut pos = Coor::zero();
    let mut furthest = 0;
    for coor in input
        .split(',')
        .filter_map(|c| c.parse::<Coor>().ok()) {
            pos += coor;
            furthest = max(furthest, pos.distance());
        }
    Ok(furthest)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("ne,ne,ne").unwrap(), 3);
        assert_eq!(part1("ne,ne,sw,sw").unwrap(), 0);
        assert_eq!(part1("ne,ne,s,s").unwrap(), 2);
        assert_eq!(part1("se,sw,se,sw,sw").unwrap(), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("ne,ne,ne").unwrap(), 3);
        assert_eq!(part2("ne,ne,sw,sw").unwrap(), 2);
        assert_eq!(part2("ne,ne,s,s").unwrap(), 2);
        assert_eq!(part2("se,sw,se,sw,sw").unwrap(), 3);
    }
}
