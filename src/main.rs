extern crate clap;
#[macro_use] extern crate failure;

use clap::{App, Arg};
use std::path::Path;

mod shared;
mod day01;
mod day02;
mod day03;
mod day04;

use shared::AppError;

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
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();

    let input = match matches.value_of("input").expect("input is required but missing") {
        "-" => shared::read_stdin(),
        filename => shared::read_file(Path::new(filename)),
    }?;

    match (
        matches.value_of("day").ok_or(AppError::InvalidProblem{})?.parse()?,
        matches.value_of("part").ok_or(AppError::InvalidProblem{})?.parse()?
    ) {
        (1, 1) => day01::part1(input),
        (1, 2) => day01::part2(input),
        (2, 1) => day02::part1(input),
        (2, 2) => day02::part2(input),
        (3, 1) => day03::part1(input),
        (3, 2) => day03::part2(input),
        (4, 1) => day04::part1(input),
        (4, 2) => day04::part2(input),
        _ => Err(format_err!("Invalid problem")),
    }
}
