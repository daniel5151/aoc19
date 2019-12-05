use std::error::Error as StdError;
use std::fmt::{self, Debug, Display};

#[derive(Debug)]
pub enum IntCodeError {
    ParseMem,
    ParseInput,
    Io(std::io::Error),
}

impl Display for IntCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::IntCodeError::*;
        match self {
            ParseMem => write!(f, "Failed to parse initial memory string"),
            ParseInput => write!(f, "Failed to parse input value"),
            Io(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl StdError for IntCodeError {}
