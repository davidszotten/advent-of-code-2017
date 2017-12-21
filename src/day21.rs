use std::collections::HashMap;
use shared::AppResult;

const START: [[char;3];3] = [['.', '#', '.'],['.', '.', '#'], ['#', '#', '#']];

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

    pub fn rotate(&self) -> Self {
        let mut new_source = vec![];
        for row in 0..self.source.len() {
            let mut new_row = vec![];
            for col in 0..self.source.len() {
                new_row.push(self.source[col][row]);
            }
            new_source.push(new_row);
        }
        Pattern {source: new_source}
    }

    pub fn flip(&self) -> Self {
        let mut new_source = vec![];
        for row in 0..self.source.len() {
            new_source.push(self.source[row].iter().cloned().rev().collect());
        }
        Pattern {source: new_source}
    }
}

fn parse(input: &str) -> HashMap<Pattern,Pattern> {
    let patterns = HashMap::new();

    patterns
}


pub fn part1(_input: &str) -> AppResult<u32> {
    Ok(0)
}


pub fn part2(_input: &str) -> AppResult<u32> {
    Ok(0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let pattern = Pattern::from_str(".#./..#/###");
        let rotated = Pattern::from_str("..#/#.#/.##");
        assert_eq!(pattern.rotate(), rotated);
    }

    #[test]
    fn test_flip() {
        let pattern = Pattern::from_str(".#./..#/###");
        let flipped = Pattern::from_str(".#./#../###");
        assert_eq!(pattern.flip(), flipped);
    }

//     #[test]
//     fn test_part1() {
//         assert_eq!(part1("../.# => ##./#../...
// .#./..#/### => #..#/..../..../#..#").unwrap(), 12);
//     }
}
