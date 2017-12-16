use shared::AppResult;
use std::collections::{HashSet, VecDeque};
use day10::sparse;
use position::Position;


// fn dense(bytes: Vec<usize>) -> Vec<usize> {
//     bytes.chunks(16).map(
//         |chunk| chunk.iter().fold(0, |x, y| x ^ y)
//     ).collect()
// }

fn dense_binary(bytes: Vec<usize>) -> String {
    bytes.chunks(16).map(
        |chunk| format!("{:08b}", chunk.iter().fold(0, |x, y| x ^ y))
    ).collect()
}

fn dense_binary_count(bytes: Vec<usize>) -> u32 {
    bytes.chunks(16).map(
        |chunk| chunk.iter().fold(0, |x, y| x ^ y).count_ones()
    ).sum()
}


pub fn part1(input: &str) -> AppResult<u32> {
    Ok(
        (0..128).map(
        |suffix| dense_binary_count(sparse(&format!("{}-{}", input, suffix)))
        ).sum())
}

fn get_coors(input: &str) -> Vec<Position> {
    let mut coors = vec![];
    for suffix in 0..128 {
        let mut row_coors: Vec<_> = dense_binary(sparse(
            &format!("{}-{}", input, suffix)
        ))
            .chars()
            .enumerate()
            .filter(|&(_, x)| x == '1')
            .map(|(i, _)| Position::new(i as i32, suffix))
            .collect();
        coors.append(&mut row_coors);
    }
    coors
}

fn count_regions(coors: Vec<Position>) -> usize {
    let mut sum = 0;
    let mut seen = HashSet::new();
    let all_coors: HashSet<_> = coors.iter().collect();
    // println!("{:?}", &all_coors);
    for coor in coors.clone() {
        if seen.contains(&coor) {
            continue;
        }
        sum += 1;
        let mut region = VecDeque::new();
        // println!("new: {:?}", &coor);
        region.push_back(coor);
        while let Some(member) = region.pop_front() {
            if seen.contains(&member) {
                continue;
            }
            // println!("   reg: {:?}", &member);
            seen.insert(member.clone());
            for neighbour in member.straight_neighbours() {
                // println!("      nei?: {:?}", &neighbour);
                if all_coors.contains(&neighbour) {
                    // println!("      nei: {:?}", &neighbour);
                    region.push_back(neighbour);
                }
            }
        }
    }
    sum
}


pub fn part2(input: &str) -> AppResult<u32> {
    Ok(count_regions(get_coors(input)) as u32)
}

/*
11010100
01010101
00001010
10101101
01101000
11001001
01000100
11010110
*/


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("flqrgnkx").unwrap(), 8108);
    }

    #[test]
    fn test_get_coors() {
        let coors = get_coors("flqrgnkx");
        assert_eq!(
            coors.into_iter().filter(|p| p.x < 4 && p.y < 4).collect::<Vec<_>>(),
            [(0, 0), (1, 0), (3, 0), (1, 1), (3, 1), (0, 3), (2, 3)]
            .iter().map(|&(x, y)| Position::new(x, y)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_count_regions() {
        assert_eq!(count_regions(
            [(0, 0), (1, 0), (3, 0), (1, 1), (3, 1), (0, 3), (2, 3)]
            .iter().map(|&(x, y)| Position::new(x, y)).collect::<Vec<_>>()),
        4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("flqrgnkx").unwrap(), 1242);
    }
}
