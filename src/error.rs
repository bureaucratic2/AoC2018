use failure::Fail;
use std::{io, num::ParseIntError};

#[derive(Fail, Debug)]
pub enum AoCError {
    #[fail(display = "{}", _0)]
    Parse(#[cause] chrono::ParseError),
    #[fail(display = "{}", _0)]
    Regex(#[cause] regex::Error),
    #[fail(display = "{}", _0)]
    ParseInt(#[cause] ParseIntError),
    #[fail(display = "{}", _0)]
    IO(#[cause] io::Error),
    #[fail(display = "{}", _0)]
    Log(#[cause] fern::InitError),
    #[fail(display = "dirty input")]
    DirtyInput,
}

impl From<ParseIntError> for AoCError {
    fn from(err: ParseIntError) -> Self {
        AoCError::ParseInt(err)
    }
}

impl From<chrono::ParseError> for AoCError {
    fn from(err: chrono::ParseError) -> Self {
        AoCError::Parse(err)
    }
}

impl From<regex::Error> for AoCError {
    fn from(err: regex::Error) -> Self {
        AoCError::Regex(err)
    }
}

impl From<io::Error> for AoCError {
    fn from(err: io::Error) -> Self {
        AoCError::IO(err)
    }
}
impl From<fern::InitError> for AoCError {
    fn from(err: fern::InitError) -> Self {
        AoCError::Log(err)
    }
}
