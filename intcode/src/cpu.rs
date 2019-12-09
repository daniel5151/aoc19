use std::collections::VecDeque;
use std::error::Error as StdError;
use std::result::Result as StdResult;

use super::error::{Error, Result};
use super::mem::Mem;

/// An Intcode interpreter.
#[derive(Debug, Clone)]
pub struct Intcode {
    mem: Mem,
    instr: usize,
    pc: usize,
    base: isize,
}

impl Intcode {
    /// Create a new Intcode machine.
    /// Returns an error if the input string is malformed.
    pub fn new(input: impl AsRef<str>) -> Result<Intcode> {
        Ok(Intcode {
            mem: Mem::new(input)?,
            instr: 0,
            pc: 0,
            base: 0,
        })
    }

    /// Reset the intcode machine to it's initial state
    pub fn reset(&mut self) {
        self.mem.reset();
        self.instr = 0;
        self.pc = 0;
        self.base = 0;
    }

    /// Return a mutable reference to the intcode machine's memory
    pub fn mem(&mut self) -> &mut Mem {
        &mut self.mem
    }

    /// Run the intcode interpreter without performing any I/O,
    /// returning an error if a read instruction is encountered
    pub fn run_headless(&mut self) -> Result<()> {
        while self.step(
            || Err("intcode cannot read input in headless mode".into()),
            |_| Ok(()),
        )? {}
        Ok(())
    }

    /// Run the intcode interpreter to completion using stdin for input, and
    /// stdout for output.
    pub fn run_interactively(&mut self) -> Result<()> {
        while self.step(
            || {
                print!("> ");
                use std::io::Read;
                let mut buf = String::new();
                std::io::stdin().read_to_string(&mut buf)?;
                Ok(buf.trim().parse::<isize>()?)
            },
            |i| {
                println!("{}", i);
                Ok(())
            },
        )? {}
        Ok(())
    }

    /// Run the intcode interpreter to completion using the provided input and
    /// output buffers. Returns an error if the input vector runs out of
    /// elements.
    pub fn run_to_completion(
        &mut self,
        input: &mut Vec<isize>,
        output: &mut Vec<isize>,
    ) -> Result<()> {
        input.reverse();

        while self.step(
            || {
                input
                    .pop()
                    .ok_or_else(|| "no more input in the input buffer".into())
            },
            |i| {
                output.push(i);
                Ok(())
            },
        )? {}
        Ok(())
    }

    /// Run the intcode interpreter with the provided input until the machine
    /// has outputted `n` values. If the machine halts, None is returned.
    /// Returns an error if the input VecDeque runs out of elements.
    pub fn run_until_output(&mut self, input: &mut VecDeque<isize>) -> Result<Option<isize>> {
        let mut output = None;
        loop {
            let running = self.step(
                || {
                    input
                        .pop_front()
                        .ok_or_else(|| "no more input in the input buffer".into())
                },
                |i| {
                    output = Some(i);
                    Ok(())
                },
            )?;

            if !running {
                return Ok(None);
            }

            if let Some(output) = output {
                return Ok(Some(output));
            }
        }
    }

    /// Return the next argument, taking into account it's addressing mode, and
    /// incrementing `self.pc` by 1.
    fn fetch_arg(&mut self) -> Result<usize> {
        let addr_mode = self.instr % 10;
        self.instr /= 10;
        let addr = match addr_mode {
            0 => (self.mem.read(self.pc)).to_addr()?,
            1 => self.pc,
            2 => (self.mem.read(self.pc) + self.base).to_addr()?,
            m => return Err(Error::InvalidAddrMode(m)),
        };
        self.pc += 1;
        Ok(addr)
    }

    /// Fetches and decodes the next instruction, updating `self.pc` accordingly
    fn fetch_decode_instr(&mut self) -> Result<Instruction> {
        // load next instruction into the instr register
        self.instr = self.mem.read(self.pc).to_raw_instr()?;
        self.pc += 1;

        // extract opcode
        let opcode = self.instr % 100;
        self.instr /= 100;

        // helper macro to fetch a typed argument
        #[rustfmt::skip]
        macro_rules! a {
            // A reference to the specified memory location
            (ptr) => {{ self.fetch_arg()? }};
            // A signed immediate value
            (imm_i) => {{ let arg = self.fetch_arg()?; self.mem.read(arg) }};
            // A unsigned immediate value
            (imm_u) => {{ let arg = self.fetch_arg()?; self.mem.read(arg).to_addr()? }};
        }

        use Instruction::*;
        let instr = match opcode {
            1 => Add_(a!(imm_i), a!(imm_i), a!(ptr)),
            2 => Mul_(a!(imm_i), a!(imm_i), a!(ptr)),
            3 => Geti(a!(ptr)),
            4 => Puti(a!(ptr)),
            5 => Jnz_(a!(imm_i), a!(imm_u)),
            6 => Jz__(a!(imm_i), a!(imm_u)),
            7 => Cmp_(a!(imm_i), a!(imm_i), a!(ptr)),
            8 => Eq__(a!(imm_i), a!(imm_i), a!(ptr)),
            9 => Setb(a!(imm_i)),
            99 => Halt,
            o => return Err(Error::InvalidOpcode(o)),
        };

        Ok(instr)
    }

    /// Step the intcode interpreter using custom input/output functions,
    /// returning `false` is the machine is halted.
    pub fn step(
        &mut self,
        input_fn: impl FnOnce() -> StdResult<isize, Box<dyn StdError>>,
        output_fn: impl FnOnce(isize) -> StdResult<(), Box<dyn StdError>>,
    ) -> Result<bool> {
        use Instruction::*;
        match self.fetch_decode_instr()? {
            Add_(a, b, dst) => self.mem.write(dst, a + b),
            Mul_(a, b, dst) => self.mem.write(dst, a * b),
            Geti(dst) => self.mem.write(dst, input_fn().map_err(Error::InputError)?),
            Puti(src) => output_fn(self.mem.read(src)).map_err(Error::OutputError)?,
            Jnz_(v, new_pc) => {
                if v != 0 {
                    self.pc = new_pc
                }
            }
            Jz__(v, new_pc) => {
                if v == 0 {
                    self.pc = new_pc
                }
            }
            Cmp_(a, b, dst) => self.mem.write(dst, (a < b) as isize),
            Eq__(a, b, dst) => self.mem.write(dst, (a == b) as isize),
            Setb(b) => self.base += b,
            Halt => return Ok(false),
        }

        Ok(true)
    }
}

#[derive(Debug)]
pub enum Instruction {
    Add_(isize, isize, usize),
    Mul_(isize, isize, usize),
    Geti(usize),
    Puti(usize),
    Jnz_(isize, usize),
    Jz__(isize, usize),
    Cmp_(isize, isize, usize),
    Eq__(isize, isize, usize),
    Setb(isize),
    Halt,
}

trait IsizeIntcodeExt {
    fn to_addr(self) -> Result<usize>;
    fn to_raw_instr(self) -> Result<usize>;
}

impl IsizeIntcodeExt for isize {
    fn to_addr(self) -> Result<usize> {
        if self < 0 {
            Err(Error::NegativeAddr)
        } else {
            Ok(self.abs() as usize)
        }
    }

    fn to_raw_instr(self) -> Result<usize> {
        if self < 0 {
            Err(Error::NegativeInstr)
        } else {
            Ok(self.abs() as usize)
        }
    }
}
