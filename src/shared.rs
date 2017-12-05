// extern crate failure;

// use std::error::{self, Error};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
// use std::num::ParseIntError;
// use std::option;

use failure::Error;

pub type AppResult<T> = Result<T, Error>;


pub fn read_stdin() -> AppResult<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;
    Ok(buffer.trim().into())
}


pub fn read_file(filename: &Path) -> AppResult<String> {
    let mut buffer = String::new();
    let mut handle = File::open(filename)?;

    handle.read_to_string(&mut buffer)?;
    Ok(buffer.trim().into())
}
