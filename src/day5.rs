use crate::DynResult;

use std::io::Read;

#[derive(Debug, Clone)]
struct IntCode {
    init_mem: Vec<isize>,
    mem: Vec<isize>,
    pc: usize,
}

impl IntCode {
    pub fn new(init_mem: Vec<isize>) -> IntCode {
        IntCode {
            init_mem: init_mem.clone(),
            mem: init_mem,
            pc: 0,
        }
    }

    pub fn reset1(&mut self, noun: isize, verb: isize) {
        self.mem.copy_from_slice(&self.init_mem);
        self.pc = 0;

        self.mem[1] = noun;
        self.mem[2] = verb;
    }

    pub fn reset(&mut self) {
        self.mem.copy_from_slice(&self.init_mem);
        self.pc = 0;
    }

    /// Returns the final value at memory location 0
    pub fn run(&mut self) -> isize {
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

        self.mem[0]
    }
}

pub fn q1(input: String, _args: &[String]) -> DynResult<()> {
    let mem = input
        .split(',')
        .map(|s| s.parse::<isize>())
        .collect::<std::result::Result<Vec<isize>, _>>()?;

    let mut intcode = IntCode::new(mem);
    intcode.reset();
    intcode.run();

    Ok(())
}

pub fn q2(input: String, _args: &[String]) -> DynResult<()> {
    // let mem = vec![
    //     3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
    //     1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
    //     1105, 1, 46, 98, 99,
    // ];

    let mem = input
        .split(',')
        .map(|s| s.parse::<isize>())
        .collect::<std::result::Result<Vec<isize>, _>>()?;

    let mut intcode = IntCode::new(mem);
    intcode.reset();
    intcode.run();

    Ok(())
}
