use std::error::Error as StdError;
use std::fmt::{self, Debug, Display};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidAddrMode(usize),
    InvalidOpcode(usize),
    Io(std::io::Error),
    NegativeAddr,
    NegativeInstr,
    ParseInput,
    ParseMem,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Error::*;
        match self {
            InvalidAddrMode(v) => write!(f, "Encountered unknown addressing mode: {}", v),
            InvalidOpcode(v) => write!(f, "Encountered unknown opcode: {}", v),
            Io(e) => write!(f, "I/O error: {}", e),
            NegativeAddr => write!(f, "Cannot address negative address"),
            NegativeInstr => write!(f, "Cannot execute negative instruction"),
            ParseInput => write!(f, "Failed to parse input value"),
            ParseMem => write!(f, "Failed to parse initial memory string"),
        }
    }
}

impl StdError for Error {}
