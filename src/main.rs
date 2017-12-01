#![feature(try_trait)]

extern crate clap;

use clap::{App, Arg};

mod shared;
mod day01;

fn main() {
    match run() {
        Ok(result) => println!("{}", result),
        Err(err) => println!("{:?}", err),
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
        .get_matches();

    match (
        matches.value_of("day")?.parse()?,
        matches.value_of("part")?.parse()?
    ) {
        (1, 1) => day01::part1(),
        (1, 2) => day01::part2(),
        _ => Err(shared::AppError::InvalidProblem),
    }
}
