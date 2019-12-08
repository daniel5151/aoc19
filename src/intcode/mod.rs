mod cpu;
mod error;
mod instr;

pub use cpu::Intcode;
pub use error::{Error, Result};

pub trait IsizeIntcodeExt {
    fn to_addr(self) -> Result<usize>;
}

impl IsizeIntcodeExt for isize {
    fn to_addr(self) -> Result<usize> {
        if self < 0 {
            Err(Error::NegativeAddr)
        } else {
            Ok(self.abs() as usize)
        }
    }
}
