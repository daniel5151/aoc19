use crate::DynResult;

#[derive(Debug, Clone)]
pub struct IntCode {
    init_mem: Vec<usize>,
    mem: Vec<usize>,
    pc: usize,
}

impl IntCode {
    pub fn new(input: String) -> DynResult<IntCode> {
        let mem = input
            .split(',')
            .map(|s| s.parse::<usize>())
            .collect::<std::result::Result<Vec<usize>, _>>()?;

        Ok(IntCode {
            init_mem: mem.clone(),
            mem,
            pc: 0,
        })
    }

    pub fn mem_len(&self) -> usize {
        self.mem.len()
    }

    pub fn read_mem(&self, addr: usize) -> Option<usize> {
        self.mem.get(addr).copied()
    }

    pub fn write_mem(&mut self, addr: usize, val: usize) -> Option<()> {
        self.mem.get_mut(addr).map(|x| *x = val)
    }

    pub fn reset(&mut self) {
        self.mem.copy_from_slice(&self.init_mem);
        self.pc = 0;
    }

    pub fn run(&mut self) {
        loop {
            if self.mem[self.pc] == 99 {
                break;
            }

            let a = self.mem[self.pc + 1];
            let b = self.mem[self.pc + 2];
            let dst = self.mem[self.pc + 3];

            match self.mem[self.pc] {
                1 => self.mem[dst] = self.mem[a] + self.mem[b],
                2 => self.mem[dst] = self.mem[a] * self.mem[b],
                _ => panic!("unexpected opcode"),
            }
            self.pc += 4;
        }
    }
}
