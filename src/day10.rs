use shared::AppResult;

pub fn part1(input: &str) -> AppResult<u32> {
    Ok(do_part1(input, 256))
}


pub fn part2(_input: &str) -> AppResult<u32> {
    Ok(0)
}

fn do_part1(input: &str, length: usize) -> u32 {
    let mut skip = 0;
    let mut pos = 0;
    let mut list: Vec<usize> = (0..length).collect();
    for length in input
            .split(',')
            .filter_map(|x| x.parse::<usize>().ok()) {

        // println!("{:?}, {}", list, length);
        list = reverse(list, length);
        list = step(list, length);
        list = step(list, skip);
        pos += length + skip;
        skip += 1;
    }
    // println!("  {:?}, {}", list, length - pos % length);
    list[length - pos % length] as u32 * list[length + 1 - pos % length] as u32
}


fn reverse(mut list: Vec<usize>, length: usize) -> Vec<usize> {
    let mut end = list.split_off(length);
    list.reverse();
    list.append(&mut end);
    list
}


fn step(mut list: Vec<usize>, step: usize) -> Vec<usize> {
    let mut end = list.split_off(step);
    end.append(&mut list);
    end
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(vec![0, 1, 2, 3, 4], 2), vec![1, 0, 2, 3, 4]);
    }

    #[test]
    fn test_step() {
        assert_eq!(step(vec![0, 1, 2, 3, 4], 2), vec![2, 3, 4, 0, 1]);
        assert_eq!(step(vec![0, 1, 2, 3, 4], 4), vec![4, 0, 1, 2, 3]);
    }

    #[test]
    fn test_do_part1() {
        assert_eq!(do_part1("3,4,1,5", 5), 12);
    }
}
