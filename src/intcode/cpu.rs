use std::collections::VecDeque;

use crate::DynResult;

use super::error::{Error, Result};
use super::instr::Instruction;
use super::IsizeIntcodeExt;

#[derive(Debug, Clone)]
pub struct Intcode {
    reset_mem: Vec<isize>,

    mem: Vec<isize>,
    pc: usize,

    in_buf: VecDeque<isize>,
    out_buf: VecDeque<isize>,
}

impl Intcode {
    /// Create a new Intcode machine, parsing the input string to intcode,
    /// returning an error if the string is malformed.
    pub fn new(input: impl AsRef<str>) -> Result<Intcode> {
        let input = input.as_ref();
        let mem = input
            .split(',')
            .map(|s| s.trim().parse::<isize>())
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|_| Error::ParseMem)?;

        Ok(Intcode {
            reset_mem: mem.clone(),
            mem,
            pc: 0,
            in_buf: VecDeque::new(),
            out_buf: VecDeque::new(),
        })
    }

    /// Reset the intcode machine to it's initial state
    pub fn reset(&mut self) {
        self.mem.copy_from_slice(&self.reset_mem);
        self.pc = 0;

        self.in_buf.clear();
        self.out_buf.clear();
    }

    /// Return the intcode machine's memory length
    pub fn mem_len(&self) -> usize {
        self.mem.len()
    }

    /// Read the integer at `addr`, retuning an error if the address is
    /// out-of-bounds
    pub fn read_mem(&self, addr: usize) -> Result<isize> {
        self.mem.get(addr).copied().ok_or(Error::OobRead)
    }

    /// Write the integer `val` to `addr`, returning an error if the address is
    /// out of bounds.
    pub fn write_mem(&mut self, addr: usize, val: isize) -> Result<()> {
        self.mem
            .get_mut(addr)
            .map(|x| *x = val)
            .ok_or(Error::OobWrite)
    }

    /// Get a mutable reference to the input buffer
    pub fn input(&mut self) -> &mut VecDeque<isize> {
        &mut self.in_buf
    }

    /// Get a mutable reference to the output buffer
    pub fn output(&mut self) -> &mut VecDeque<isize> {
        &mut self.out_buf
    }

    /// Run the intcode interpreter without performing any I/O,
    /// returning an error if a read instruction is encountered
    pub fn run_headless(&mut self) -> Result<()> {
        while self.step(
            || Err("intcode cannot read input in headless mode".into()),
            |_| (),
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
            |i| println!("{}", i),
        )? {}
        Ok(())
    }

    /// Run the intcode interpreter to completion using the provided input
    /// vector. Returns all output in a single output vector. Returns an error
    /// if the input Vec runs out of elements.
    pub fn run_to_completion(&mut self, mut input: Vec<isize>) -> Result<()> {
        input.reverse();
        while self.step(
            || {
                input
                    .pop()
                    .ok_or_else(|| "no more input in the input buffer".into())
            },
            |_| (),
        )? {}
        Ok(())
    }

    /// Run the intcode interpreter with the provided input until the machine
    /// has outputted `n` values. If the machine halts, None is returned.
    /// Returns an error if the input VecDeque runs out of elements.
    pub fn run_until_output(&mut self) -> Result<Option<isize>> {
        let mut output = None;
        loop {
            let running = self.step(
                || Err("no more input in the input buffer".into()),
                |i| output = Some(i),
            )?;

            if !running {
                return Ok(None);
            }

            if let Some(output) = output {
                assert!(self.out_buf.pop_front() == Some(output));
                return Ok(Some(output));
            }
        }
    }

    /// Step the intcode interpreter using custom input/output functions,
    /// returning `false` is the machine is halted.
    pub fn step(
        &mut self,
        mut get_input: impl FnMut() -> DynResult<isize>,
        mut on_output: impl FnMut(isize),
    ) -> Result<(bool)> {
        let (instr, instr_len) = Instruction::decode(self.pc, &self.mem)?;
        self.pc += instr_len;

        use Instruction::*;
        match instr {
            Halt => return Ok(false),
            // 1 arg
            ReadInt(dst) => {
                self.mem[dst] = match self.in_buf.pop_front() {
                    Some(v) => v,
                    // last-ditch attempt to get some input from the user
                    None => get_input().map_err(Error::InputError)?,
                }
            }
            WriteInt(src) => {
                let val = self.mem[src];
                self.out_buf.push_back(val);
                on_output(val)
            }
            // 2 arg
            Jnz(v, pc) => {
                if self.mem[v] != 0 {
                    self.pc = self.mem[pc].to_addr()?
                }
            }
            Jz(v, pc) => {
                if self.mem[v] == 0 {
                    self.pc = self.mem[pc].to_addr()?;
                }
            }
            // 3 arg
            Add(a, b, dst) => self.mem[dst] = self.mem[a] + self.mem[b],
            Cmp(a, b, dst) => self.mem[dst] = (self.mem[a] < self.mem[b]) as isize,
            Eq(a, b, dst) => self.mem[dst] = (self.mem[a] == self.mem[b]) as isize,
            Mul(a, b, dst) => self.mem[dst] = self.mem[a] * self.mem[b],
        }

        Ok(true)
    }
}
