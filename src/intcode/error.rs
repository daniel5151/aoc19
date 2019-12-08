use std::error::Error as StdError;
use std::fmt::{self, Debug, Display};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InputError(Box<dyn StdError>),
    OobRead,
    OobWrite,
    InvalidAddrMode(usize),
    InvalidOpcode(usize),
    NegativeAddr,
    NegativeInstr,
    ParseMem,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Error::*;
        match self {
            InvalidAddrMode(v) => write!(f, "Encountered unknown addressing mode: {}", v),
            InvalidOpcode(v) => write!(f, "Encountered unknown opcode: {}", v),
            InputError(e) => write!(f, "Could not read input: {}", e),
            NegativeAddr => write!(f, "Cannot address negative address"),
            NegativeInstr => write!(f, "Cannot execute negative instruction"),
            ParseMem => write!(f, "Failed to parse initial memory string"),
            OobRead => write!(f, "Attempted to read from an address that's out of bounds"),
            OobWrite => write!(f, "Attempted to write to an address that's out of bounds"),
        }
    }
}

impl StdError for Error {}
