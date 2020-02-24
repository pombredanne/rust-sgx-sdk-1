//!error system
use std::{error::Error as StdErr, fmt, io, num, str};
use std::prelude::v1::*;

#[derive(Debug, PartialEq)]
pub enum ParseErr {
    Utf8(str::Utf8Error),
    Int(num::ParseIntError),
    StatusErr,
    HeadersErr,
    UriErr,
    Invalid,
    Empty,
}

impl fmt::Display for ParseErr {
    #![allow(deprecated)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseErr: {}", self.description())
    }
}

impl StdErr for ParseErr {
    fn description(&self) -> &str {
        use self::ParseErr::*;

        match self {
            Utf8(_) => "invalid character",
            Int(_) => "cannot parse number",
            Invalid => "invalid value",
            Empty => "nothing to parse",
            StatusErr => "status line contains invalid values",
            HeadersErr => "headers contain invalid values",
            UriErr => "uri contains invalid characters",
        }
    }
}

impl From<num::ParseIntError> for ParseErr {
    fn from(e: num::ParseIntError) -> Self {
        ParseErr::Int(e)
    }
}

impl From<str::Utf8Error> for ParseErr {
    fn from(e: str::Utf8Error) -> Self {
        ParseErr::Utf8(e)
    }
}

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Parse(ParseErr),
    Tls,
}

impl fmt::Display for Error {
    #![allow(deprecated)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.description())
    }
}

impl StdErr for Error {
    #![allow(deprecated)]
    fn description(&self) -> &str {
        use self::Error::*;

        match self {
            IO(_) => "IO error",
            Parse(e) => e.description(),
            Tls => "TLS error",
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<ParseErr> for Error {
    fn from(e: ParseErr) -> Self {
        Error::Parse(e)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(e: str::Utf8Error) -> Self {
        Error::Parse(ParseErr::Utf8(e))
    }
}
