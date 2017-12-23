use std::ops;
use std::collections::HashMap;
use shared::AppResult;

// const START: [[char;3];3] = [['.', '#', '.'],['.', '.', '#'], ['#', '#', '#']];
const START: &str = ".#./..#/###";

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pattern {
    source: Vec<Vec<char>>
}

impl Pattern {
    pub fn from_str(source: &str) -> Self {
        let source_vec = source
            .split('/')
            .map(|row| row.chars().collect::<Vec<_>>())
            .collect();
        Pattern{source: source_vec}
    }

    pub fn as_str(&self) -> String {
        self.source.iter().map(|v| v.into_iter().collect::<String>()).collect::<Vec<_>>().join("/")
    }

    pub fn rotate(&self) -> Self {
        let mut new_source = vec![];
        for row in 0..self.source.len() {
            let mut new_row = vec![];
            for col in 0..self.source.len() {
                new_row.push(self.source[self.source.len() - col - 1][row]);
            }
            new_source.push(new_row);
        }
        Pattern {source: new_source}
    }

    pub fn fliplr(&self) -> Self {
        let mut new_source = vec![];
        for row in 0..self.source.len() {
            new_source.push(self.source[row].iter().cloned().rev().collect());
        }
        Pattern {source: new_source}
    }

    pub fn flipud(&self) -> Self {
        self.rotate().fliplr().rotate().rotate().rotate()
    }

    pub fn split(&self, side_length: usize) -> Vec<Vec<Pattern>> {
        let mut res = vec![];
        for entry_row in 0..self.source.len() / side_length {
            let mut res_row = vec![];
            for entry_col in 0..self.source.len() / side_length {
                let mut new_source = vec![];
                for row in 0..side_length {
                    let mut new_row = vec![];
                    for col in 0..side_length {
                        new_row.push(
                            self.source[entry_col * side_length + col][entry_row * side_length + row]
                        );
                    }
                    new_source.push(new_row);
                }
                res_row.push(Pattern {source: new_source});
            }
            res.push(res_row);
        }
        res
    }

    pub fn combine(grid: Vec<Vec<Pattern>>) -> Self {
        let sub_pattern = &grid[0][0];
        let sub_len = sub_pattern.source.len();
        let mut new_source = vec![];

        for grid_row in 0..grid.len() * sub_len{
            let mut new_row = vec![];
            for grid_col in 0..grid.len() * sub_len {
                new_row.push(
                    grid[grid_col / sub_len][grid_row / sub_len]
                    .source[grid_col % sub_len][grid_row % sub_len]
                );
            }
            new_source.push(new_row);
        }
        Pattern {source: new_source}

    }

    pub fn replace(&self, replacements: &HashMap<Pattern, Pattern>) -> Pattern {
        // println!("looking for {}", self.as_str());
        for (key, ref value) in replacements.iter() {
            // println!("next: {}", key.as_str());
            let mut possibility = self.clone();
            for _ in 0..3 {
                // println!("trying {}", possibility.as_str());
                if possibility == *key || possibility.fliplr() == *key || possibility.flipud() == *key {
                    // println!("found match");
                    return (*value).clone();
                }
                // println!("rotating {} to {}", possibility.as_str(), possibility.rotate().as_str());
                possibility = possibility.rotate();
            }
        }
        panic!("no match found: {:?}", self);
    }

    pub fn count(&self) -> u32 {
        self.source.iter().map(|line| line.iter().filter(|&c| *c == '#').count()).sum::<usize>() as u32
    }
}

impl ops::Index<(usize, usize)> for Pattern {
    type Output = char;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.source[y][x]
    }
}


fn parse(input: &str) -> HashMap<Pattern,Pattern> {
    let mut patterns = HashMap::new();
    for line in input.split('\n') {
        let mut parts = line.split(' ');
        let lhs = parts.nth(0).expect("invalid pattern");
        let rhs = parts.nth(1).expect("invalid pattern");
        patterns.insert(Pattern::from_str(lhs), Pattern::from_str(rhs));
    }

    patterns
}

fn run(input: &str, iterations: usize) -> u32 {
    let replacements = parse(input);
    let mut current = Pattern::from_str(START);
    println!("current: {}", current.as_str());
    for it in 0..iterations {
        println!("iteration: {}", it);
        // let foo = Pattern::from_str("#..#/..../..../#..#");
        let side_length = if current.source.len() % 2 == 0 {2} else {3};
        // println!("{:?}", current.split(side_length));
        let parts: Vec<_> = current.split(side_length).iter().map(
            |v| v.iter().map(|p| p.replace(&replacements)).collect::<Vec<_>>()
        ).collect();
        current = Pattern::combine(parts);
        // println!("{:?}", parts);
    }
    current.count()
}


pub fn part1(input: &str) -> AppResult<u32> {
    Ok(run(input, 5))
}


pub fn part2(input: &str) -> AppResult<u32> {
    Ok(run(input, 18))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        /*
        .#.      #..
        ..#  ->  #.#
        ###      ##.

        */
        let pattern = Pattern::from_str(".#./..#/###");
        let rotated = Pattern::from_str("#../#.#/##.");
        assert_eq!(pattern.rotate(), rotated);
    }

    #[test]
    fn test_rotate2x2() {
        /*
        .#      .#
        ..  ->  #.

        */
        // let pattern = Pattern::from_str(".#/..");
        // let rotated = Pattern::from_str("../#.#/.##");
        // assert_eq!(pattern.rotate(), rotated);
    }

    #[test]
    fn test_fliplr() {
        let pattern = Pattern::from_str(".#./..#/###");
        let flipped = Pattern::from_str(".#./#../###");
        assert_eq!(pattern.fliplr(), flipped);
    }

    #[test]
    fn test_flipud() {
        let pattern = Pattern::from_str(".#./..#/###");
        let flipped = Pattern::from_str("###/..#/.#.");
        assert_eq!(pattern.flipud(), flipped);
    }

    #[test]
    fn test_part1() {
        println!("");
        assert_eq!(run("../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#", 2), 12);
    }
}
