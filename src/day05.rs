use shared::AppResult;

pub fn part1(input: &str) -> AppResult<u32> {
    let mut instructions: Vec<i32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let mut steps = 0;
    let mut pos: i32 = 0;
    while pos >= 0 && pos < instructions.len() as i32 {
        let offset = instructions[pos as usize];
        let next = pos + offset;
        instructions[pos as usize] += 1;
        pos = next;
        steps += 1;
        // println!("{:?}", instructions);
    }
    Ok(steps)
}


pub fn part2(input: &str) -> AppResult<u32> {
    let mut instructions: Vec<i32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let mut steps = 0;
    let mut pos: i32 = 0;
    while pos >= 0 && pos < instructions.len() as i32 {
        let offset = instructions[pos as usize];
        let next = pos + offset;
        if offset >= 3 {
            instructions[pos as usize] -= 1;
        } else {
            instructions[pos as usize] += 1;
        }
        pos = next;
        steps += 1;
        // println!("{:?}", instructions);
    }
    Ok(steps)
}
