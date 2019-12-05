use std::io::{BufRead, Write};

pub use super::error::IntCodeError;

fn read_num(r: &mut impl BufRead) -> Result<isize, IntCodeError> {
    let mut buf = String::new();
    r.read_to_string(&mut buf).map_err(IntCodeError::Io)?;
    buf.trim()
        .parse::<isize>()
        .map_err(|_| IntCodeError::ParseInput)
}

#[derive(Debug, Clone)]
pub struct IntCode {
    init_mem: Vec<isize>,
    mem: Vec<isize>,
    pc: usize,
}

impl IntCode {
    pub fn new(input: String) -> Result<IntCode, IntCodeError> {
        let mem = input
            .split(',')
            .map(|s| s.trim().parse::<isize>())
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|_| IntCodeError::ParseMem)?;

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

    /// Runs the intcode interpretter using stdin and stdout
    pub fn run(&mut self) -> Result<(), IntCodeError> {
        self.run_with_io(std::io::stdin().lock(), std::io::stdout().lock())
    }

    /// Runs the intcode interpretter using the specified input stream,
    /// outputting to stdout
    pub fn run_with_input(&mut self, input: impl BufRead) -> Result<(), IntCodeError> {
        self.run_with_io(input, std::io::stdout().lock())
    }

    /// Runs the intcode interpretter using the specified input and output
    /// streams
    pub fn run_with_io(
        &mut self,
        mut input: impl BufRead,
        mut output: impl Write,
    ) -> Result<(), IntCodeError> {
        loop {
            let instr = self.mem[self.pc];
            let opcode = instr % 100;

            let addr = |i: usize| {
                let addr_mode = (instr / 100) / (10_isize.pow((i - 1) as u32)) % 10;
                match addr_mode {
                    0 => self.mem[self.pc + i] as usize,
                    1 => self.pc + i,
                    _ => unimplemented!(),
                }
            };

            match opcode {
                99 => break,
                1 => {
                    let a = addr(1);
                    let b = addr(2);
                    let dst = addr(3) as usize;
                    self.mem[dst] = self.mem[a] + self.mem[b];
                }
                2 => {
                    let a = addr(1);
                    let b = addr(2);
                    let dst = addr(3) as usize;
                    self.mem[dst] = self.mem[a] * self.mem[b];
                }
                3 => {
                    let dst = addr(1) as usize;
                    self.mem[dst] = read_num(&mut input)?;
                }
                4 => writeln!(output, "{}", self.mem[addr(1)]).map_err(IntCodeError::Io)?,
                5 => {
                    let a = addr(1);
                    let b = addr(2);
                    if self.mem[a] != 0 {
                        self.pc = self.mem[b] as usize;
                        self.pc -= 3; // hack
                    }
                }
                6 => {
                    let a = addr(1);
                    let b = addr(2);
                    if self.mem[a] == 0 {
                        self.pc = self.mem[b] as usize;
                        self.pc -= 3; // hack
                    }
                }
                7 => {
                    let a = addr(1);
                    let b = addr(2);
                    let dst = addr(3) as usize;
                    self.mem[dst] = (self.mem[a] < self.mem[b]) as isize;
                }
                8 => {
                    let a = addr(1);
                    let b = addr(2);
                    let dst = addr(3) as usize;
                    self.mem[dst] = (self.mem[a] == self.mem[b]) as isize;
                }
                _ => panic!("unexpected opcode"),
            }

            self.pc += match opcode {
                1 | 2 | 7 | 8 => 4,
                3 | 4 => 2,
                5 | 6 => 3,
                _ => unimplemented!(),
            };
        }

        Ok(())
    }
}
