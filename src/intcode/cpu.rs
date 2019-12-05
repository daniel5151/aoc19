use std::io::{BufRead, Write};

use super::error::{Error, Result};
use super::instr::Instruction;
use super::IsizeIntCodeExt;

#[derive(Debug, Clone)]
pub struct IntCode {
    init_mem: Vec<isize>,
    mem: Vec<isize>,
    pc: usize,
}

impl IntCode {
    pub fn new(input: String) -> Result<IntCode> {
        let mem = input
            .split(',')
            .map(|s| s.trim().parse::<isize>())
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|_| Error::ParseMem)?;

        Ok(IntCode {
            init_mem: mem.clone(),
            mem,
            pc: 0,
        })
    }

    pub fn mem_len(&self) -> usize {
        self.mem.len()
    }

    pub fn read_mem(&self, addr: usize) -> Option<isize> {
        self.mem.get(addr).copied()
    }

    pub fn write_mem(&mut self, addr: usize, val: isize) -> Option<()> {
        self.mem.get_mut(addr).map(|x| *x = val)
    }

    pub fn reset(&mut self) {
        self.mem.copy_from_slice(&self.init_mem);
        self.pc = 0;
    }

    /// Runs the intcode interpreter using stdin and stdout
    pub fn run(&mut self) -> Result<()> {
        self.run_with_io(std::io::stdin().lock(), std::io::stdout().lock())
    }

    /// Runs the intcode interpreter using the specified input stream,
    /// outputting to stdout
    pub fn run_with_input(&mut self, input: impl BufRead) -> Result<()> {
        self.run_with_io(input, std::io::stdout().lock())
    }

    /// Runs the intcode interpreter using the specified input and output
    /// streams
    pub fn run_with_io(&mut self, mut input: impl BufRead, mut output: impl Write) -> Result<()> {
        loop {
            let (instr, instr_len) = Instruction::decode(self.pc, &self.mem)?;
            self.pc += instr_len;

            use Instruction::*;
            match instr {
                Halt => break,
                // 1 arg
                ReadInt(dst) => self.mem[dst] = read_num(&mut input)?,
                WriteInt(src) => writeln!(output, "{}", self.mem[src]).map_err(Error::Io)?,
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
        }

        Ok(())
    }
}

fn read_num(r: &mut impl BufRead) -> Result<isize> {
    let mut buf = String::new();
    r.read_to_string(&mut buf).map_err(Error::Io)?;
    buf.trim().parse::<isize>().map_err(|_| Error::ParseInput)
}
