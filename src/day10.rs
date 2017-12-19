use shared::AppResult;

pub fn part1(input: &str) -> AppResult<u32> {
    Ok(do_part1(input, 256))
}


pub fn part2(input: &str) -> AppResult<u32> {
    println!("{}", hash(input));
    Ok(0)
}

fn setup(input: &str) -> Vec<usize> {
    [input.trim().as_bytes(), &[17, 31, 73, 47, 23]]
        .concat()
        .iter()
        .map(|&x| x as usize)
        .collect()
}

fn get_lengths(input: &str) -> Vec<usize> {
    input
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect()
}

fn do_part1(input: &str, length: usize) -> u32 {
    let list: Vec<usize> = (0..length).collect();

    let (list, pos, _) = do_round(list, &get_lengths(input), 0, 0);
    let list_size = list.len();

    list[list_size - pos % list_size] as u32 *
    list[list_size + 1 - pos % list_size] as u32
}

pub fn sparse(input: &str) -> Vec<usize> {
    let mut list: Vec<usize> = (0..256).collect();

    let lengths = setup(input);
    // println!("{:?}", lengths);
    let mut pos = 0;
    let mut skip = 0;
    for _ in 0..64 {
        // println!("hash pos, skip: {}, {}", pos, skip);
        let (n_list, n_pos, n_skip) = do_round(list, &lengths, pos, skip);
        list = n_list;
        pos = n_pos;
        skip = n_skip;
    }
    // println!("{:?}", list);
    let list_len = list.len();
    let list = step(list, list_len - pos);
    list
}

fn hash(input: &str) -> String {
    dense(sparse(input))
}

fn dense(bytes: Vec<usize>) -> String {
    bytes.chunks(16).map(
        |chunk| format!("{:02x}", chunk.iter().fold(0, |x, y| x ^ y))
    ).collect()
}


fn do_round(
    mut list: Vec<usize>, lengths: &[usize], initial_pos: usize, initial_skip: usize
) -> (Vec<usize>, usize, usize) {
    let mut skip = initial_skip;
    let mut pos = initial_pos;
    let list_len = list.len();
    for length in lengths {

        list[..*length % list_len].reverse();
        list.rotate((*length + skip) % list_len);
        pos += length + skip;
        skip += 1;
    }
    // println!("  {:?}, {}", list, length - pos % length);
    // println!("{}, {}", pos % list_len, skip % list_len);
    (
        list,
        pos % list_len,
        skip % list_len,
    )
}


fn step(mut list: Vec<usize>, step: usize) -> Vec<usize> {
    list.rotate(step);
    list
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        assert_eq!(step(vec![0, 1, 2, 3, 4], 2), vec![2, 3, 4, 0, 1]);
        assert_eq!(step(vec![0, 1, 2, 3, 4], 4), vec![4, 0, 1, 2, 3]);
    }

    #[test]
    fn test_do_part1() {
        assert_eq!(do_part1("3,4,1,5", 5), 12);
    }

    #[test]
    fn test_setup() {
        assert_eq!(setup("1,2,3"), [49, 44, 50, 44, 51, 17, 31, 73, 47, 23]);
    }

    #[test]
    fn test_dense() {
        assert_eq!(dense(vec![65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22]), "40");
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
