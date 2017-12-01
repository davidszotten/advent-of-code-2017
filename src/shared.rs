// extern crate failure;

// use std::error::{self, Error};
use std::io::{self, Read};
use std::num::ParseIntError;
use std::option;


#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    ParseInt(ParseIntError),
    None(option::NoneError),
    InvalidProblem,
}

pub type AppResult<T> = Result<T, AppError>;


impl From<io::Error> for AppError {
    fn from(err: io::Error) -> AppError {
        AppError::Io(err)
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> AppError {
        AppError::ParseInt(err)
    }
}

impl From<option::NoneError> for AppError {
    fn from(err: option::NoneError) -> AppError {
        AppError::None(err)
    }
}

pub fn read_stdin() -> AppResult<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;
    Ok(buffer.trim().into())
}
