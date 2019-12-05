mod cpu;
mod error;
mod instr;

pub use cpu::IntCode;
pub use error::{Error, Result};

pub trait IsizeIntCodeExt {
    fn to_addr(self) -> Result<usize>;
}

impl IsizeIntCodeExt for isize {
    fn to_addr(self) -> Result<usize> {
        if self < 0 {
            Err(Error::NegativeAddr)
        } else {
            Ok(self.abs() as usize)
        }
    }
}
