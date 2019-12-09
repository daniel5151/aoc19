use std::collections::VecDeque;

use crate::DynResult;

use super::error::{Error, Result};
use super::mem::Mem;
use super::IsizeIntcodeExt;

/// An Intcode interpreter.
#[derive(Debug, Clone)]
pub struct Intcode {
    mem: Mem,
    pc: usize,
    base: isize,
}

impl Intcode {
    /// Create a new Intcode machine.
    /// Returns an error if the input string is malformed.
    pub fn new(input: impl AsRef<str>) -> Result<Intcode> {
        Ok(Intcode {
            mem: Mem::new(input)?,
            pc: 0,
            base: 0,
        })
    }

    /// Reset the intcode machine to it's initial state
    pub fn reset(&mut self) {
        self.mem.reset();
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

    /// Fetches and decodes the next instruction, updating `self.pc` accordingly
    fn fetch_decode(&mut self) -> Result<Instruction> {
        let instr = self.mem.read(self.pc);
        let instr = if instr < 0 {
            return Err(Error::NegativeInstr);
        } else {
            self.mem.read(self.pc).abs() as usize
        };

        let mut instr_len = 1;
        // Return the memory address of the next argument
        let mut arg = || {
            let addr_mode = (instr / 100) / (10_usize.pow((instr_len - 1) as u32)) % 10;
            let addr = match addr_mode {
                0 => (self.mem.read(self.pc + instr_len)).to_addr()?,
                1 => self.pc + instr_len,
                2 => (self.mem.read(self.pc + instr_len) + self.base).to_addr()?,
                m => return Err(Error::InvalidAddrMode(m)),
            };
            instr_len += 1;
            Ok(addr)
        };

        #[rustfmt::skip]
        macro_rules! a {
            (addr) => { arg()? };
            (_int) => { self.mem.read(arg()?) };
            (uint) => { self.mem.read(arg()?).to_addr()? };
        }

        use Instruction::*;
        let opcode = instr % 100;
        let instr = match opcode {
            1 => Add_(a!(_int), a!(_int), a!(addr)),
            2 => Mul_(a!(_int), a!(_int), a!(addr)),
            3 => Geti(a!(addr)),
            4 => Puti(a!(addr)),
            5 => Jnz_(a!(_int), a!(uint)),
            6 => Jz__(a!(_int), a!(uint)),
            7 => Cmp_(a!(_int), a!(_int), a!(addr)),
            8 => Eq__(a!(_int), a!(_int), a!(addr)),
            9 => Setb(a!(_int)),
            99 => Halt,
            o => return Err(Error::InvalidOpcode(o)),
        };

        self.pc += instr_len;

        Ok(instr)
    }

    /// Step the intcode interpreter using custom input/output functions,
    /// returning `false` is the machine is halted.
    pub fn step(
        &mut self,
        mut input_fn: impl FnMut() -> DynResult<isize>,
        mut output_fn: impl FnMut(isize) -> DynResult<()>,
    ) -> Result<(bool)> {
        use Instruction::*;
        match self.fetch_decode()? {
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
