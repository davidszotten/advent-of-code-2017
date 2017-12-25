#![feature(conservative_impl_trait)]
#![feature(match_default_bindings)]
#![feature(slice_rotate)]
#![feature(universal_impl_trait)]


extern crate clap;
#[macro_use] extern crate failure;
// extern crate itertools;
#[macro_use] extern crate nom;

use clap::{App, Arg};
use std::path::Path;

mod direction;
mod parsers;
mod position;
mod shared;
mod tablet;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() {
    match run() {
        Ok(result) => println!("{}", result),
        Err(err) => println!("{}", err),
    };
}


fn run() -> shared::AppResult<u32> {
    let matches = App::new("adventofcode")
        .arg(Arg::with_name("day")
            .short("d")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("part")
            .short("p")
            .takes_value(true)
            .default_value("1")
            .possible_values(&["1", "2"])
        )
        .arg(Arg::with_name("input")
            .help("Sets the input file to use, or `-` for stdin")
            .required(true)
            .index(1))
        .get_matches();

    let input = match matches
        .value_of("input")
        .expect("input is required but missing")
    {
        "-" => shared::read_stdin(),
        filename => shared::read_file(Path::new(filename)),
    }?;

    match (
        matches.value_of("day").ok_or(format_err!("Invalid day"))?.parse()?,
        matches.value_of("part").ok_or(format_err!("Invalid part"))?.parse()?
    ) {
        (1, 1) => day01::part1(&input),
        (1, 2) => day01::part2(&input),
        (2, 1) => day02::part1(&input),
        (2, 2) => day02::part2(&input),
        (3, 1) => day03::part1(&input),
        (3, 2) => day03::part2(&input),
        (4, 1) => day04::part1(&input),
        (4, 2) => day04::part2(&input),
        (5, 1) => day05::part1(&input),
        (5, 2) => day05::part2(&input),
        (6, 1) => day06::part1(&input),
        (6, 2) => day06::part2(&input),
        (7, 1) => day07::part1(&input),
        (7, 2) => day07::part2(&input),
        (8, 1) => day08::part1(&input),
        (8, 2) => day08::part2(&input),
        (9, 1) => day09::part1(&input),
        (9, 2) => day09::part2(&input),
        (10, 1) => day10::part1(&input),
        (10, 2) => day10::part2(&input),
        (11, 1) => day11::part1(&input),
        (11, 2) => day11::part2(&input),
        (12, 1) => day12::part1(&input),
        (12, 2) => day12::part2(&input),
        (13, 1) => day13::part1(&input),
        (13, 2) => day13::part2(&input),
        (14, 1) => day14::part1(&input),
        (14, 2) => day14::part2(&input),
        (15, 1) => day15::part1(&input),
        (15, 2) => day15::part2(&input),
        (16, 1) => day16::part1(&input),
        (16, 2) => day16::part2(&input),
        (17, 1) => day17::part1(&input),
        (17, 2) => day17::part2(&input),
        (18, 1) => day18::part1(&input),
        (18, 2) => day18::part2(&input),
        (19, 1) => day19::part1(&input),
        (19, 2) => day19::part2(&input),
        (20, 1) => day20::part1(&input),
        (20, 2) => day20::part2(&input),
        (21, 1) => day21::part1(&input),
        (21, 2) => day21::part2(&input),
        (22, 1) => day22::part1(&input),
        (22, 2) => day22::part2(&input),
        (23, 1) => day23::part1(&input),
        (23, 2) => day23::part2(&input),
        (24, 1) => day24::part1(&input),
        (24, 2) => day24::part2(&input),
        (25, 1) => day25::part1(&input),
        (25, 2) => day25::part2(&input),
        (d, 1) => bail!("Invalid problem `{}`", d),
        (d, 2) => bail!("Invalid problem `{}`", d),
        p => bail!("Invalid problem spec `{:?}`", p),
    }
}
