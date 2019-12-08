use super::error::{Error, Result};
use super::IsizeIntcodeExt;

#[derive(Debug)]
pub enum Instruction {
    // 0 args
    Halt,
    // 1 arg
    ReadInt(usize),
    WriteInt(usize),
    // 2 arg
    Jnz(usize, usize),
    Jz(usize, usize),
    // 3 arg
    Add(usize, usize, usize),
    Cmp(usize, usize, usize),
    Eq(usize, usize, usize),
    Mul(usize, usize, usize),
}

impl Instruction {
    pub fn decode(pc: usize, mem: &[isize]) -> Result<(Instruction, usize)> {
        let instr = mem[pc];
        let instr = if instr < 0 {
            return Err(Error::NegativeInstr);
        } else {
            mem[pc].abs() as usize
        };

        let mut instr_len = 1;
        // Return the memory address of the next argument, and increments the instr_len
        let mut next_addr = || {
            let i = instr_len; // just to keep things succinct
            let addr_mode = (instr / 100) / (10_usize.pow((i - 1) as u32)) % 10;
            let addr = match addr_mode {
                0 => mem[pc + i].to_addr(),
                1 => Ok(pc + i),
                m => Err(Error::InvalidAddrMode(m)),
            };
            instr_len += 1;
            addr
        };

        use Instruction::*;
        let opcode = instr % 100;
        let instr = match opcode {
            1 => Add(next_addr()?, next_addr()?, next_addr()?),
            2 => Mul(next_addr()?, next_addr()?, next_addr()?),
            3 => ReadInt(next_addr()?),
            4 => WriteInt(next_addr()?),
            5 => Jnz(next_addr()?, next_addr()?),
            6 => Jz(next_addr()?, next_addr()?),
            7 => Cmp(next_addr()?, next_addr()?, next_addr()?),
            8 => Eq(next_addr()?, next_addr()?, next_addr()?),
            99 => Halt,
            o => return Err(Error::InvalidOpcode(o)),
        };

        Ok((instr, instr_len))
    }
}
