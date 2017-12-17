use shared::AppResult;

const LOWER16: u64 = 0b1111_1111_1111_1111;
const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;
const QUOT: u64 = 2147483647;


fn do_rounds(a: u64, b: u64) -> u32 {
    let mut matches = 0;
    let mut a = a;
    let mut b = b;
    for _ in 0..40_000_000 {
        a = (a * FACTOR_A) % QUOT;
        b = (b * FACTOR_B) % QUOT;
        if (a & LOWER16) == (b & LOWER16) {
            matches += 1;
        }
        // println!("{} {}", a, b);
    }
    matches
}


fn do_rounds_v2(a: u64, b: u64) -> u32 {
    let mut matches = 0;
    let mut a = a;
    let mut b = b;
    for _ in 0..5_000_000 {
        a = (a * FACTOR_A) % QUOT;
        while a % 4 != 0 {
            a = (a * FACTOR_A) % QUOT;
        }
        b = (b * FACTOR_B) % QUOT;
        while b % 8 != 0 {
            b = (b * FACTOR_B) % QUOT;
        }
        if (a & LOWER16) == (b & LOWER16) {
            matches += 1;
        }
        // println!("{} {}", a, b);
    }
    matches
}

pub fn part1(_input: &str) -> AppResult<u32> {
    Ok(do_rounds(722, 354))
}


pub fn part2(_input: &str) -> AppResult<u32> {
    Ok(do_rounds_v2(722, 354))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test] #[ignore]
    fn test_do_rounds() {
        assert_eq!(do_rounds(65, 8921), 588);
    }

    #[test] #[ignore]
    fn test_do_rounds_v2() {
        assert_eq!(do_rounds_v2(65, 8921), 309);
    }
}
