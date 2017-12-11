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
    walk(input).map(|c| c.distance())
}


fn walk(input: &str) -> AppResult<Coor> {
    let directions: Vec<Coor> = input
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();
    let mut start = Coor::new(0, 0, 0);
    for direction in directions {
        start += direction;
    }
    Ok(start)
}


pub fn part2(input: &str) -> AppResult<u32> {
    let directions: Vec<Coor> = input
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();
    let mut start = Coor::new(0, 0, 0);
    let mut furthest = 0;
    for direction in directions {
        start += direction;
        furthest = max(furthest, start.distance());
    }
    Ok(furthest)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walk() {
        assert_eq!(walk("ne,ne,ne").unwrap().distance(), 3);
        assert_eq!(walk("ne,ne,sw,sw").unwrap().distance(), 0);
        assert_eq!(walk("ne,ne,s,s").unwrap().distance(), 2);
        assert_eq!(walk("se,sw,se,sw,sw").unwrap().distance(), 3);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("").unwrap(), 0);
    }
}
