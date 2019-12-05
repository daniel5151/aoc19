use crate::DynResult;

use std::io::Read;

#[derive(Debug, Clone)]
pub struct IntCode {
    init_mem: Vec<isize>,
    mem: Vec<isize>,
    pc: usize,
}

impl IntCode {
    pub fn new(input: String) -> DynResult<IntCode> {
        let mem = input
            .split(',')
            .map(|s| s.parse::<isize>())
            .collect::<std::result::Result<Vec<isize>, _>>()?;

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

    /// Returns the final value at memory location 0
    pub fn run(&mut self) {
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

            let mut input = String::new();

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
                    self.mem[dst] = {
                        // FIXME
                        std::io::stdin().read_to_string(&mut input).unwrap();
                        input.trim().parse::<isize>().unwrap()
                    }
                }
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
                4 => println!("{}", self.mem[addr(1)]),
                _ => panic!("unexpected opcode"),
            }

            self.pc += match opcode {
                1 | 2 | 7 | 8 => 4,
                3 | 4 => 2,
                5 | 6 => 3,
                _ => unimplemented!(),
            };
        }
    }
}
