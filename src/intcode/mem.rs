use std::cell::RefCell;

use super::{Error, Result};

/// An Intcode machine memory module.
#[derive(Debug, Clone)]
pub struct Mem {
    orig_mem: Vec<isize>,
    mem: RefCell<Vec<isize>>,
}

impl Mem {
    /// Create a new Intcode machine Memory module.
    /// Returns an error if the input string is malformed.
    pub fn new(input: impl AsRef<str>) -> Result<Mem> {
        let input = input.as_ref();
        let mem = input
            .split(',')
            .map(|s| s.trim().parse::<isize>())
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|_| Error::ParseMem)?;

        Ok(Mem {
            orig_mem: mem.clone(),
            mem: RefCell::new(mem),
        })
    }

    /// Resets memory back to it's initial state
    pub fn reset(&mut self) {
        let mem = self.mem.get_mut();
        *mem = self.orig_mem.clone();
    }

    /// Returns the current memory length
    pub fn len(&self) -> usize {
        self.mem.borrow().len()
    }

    /// Read the integer at `addr`, silently growing memory if the addr hasn't
    /// been initialized yet.
    pub fn read(&self, addr: usize) -> isize {
        let mut mem = self.mem.borrow_mut();
        match mem.get(addr) {
            Some(v) => *v,
            None => {
                mem.resize(addr + 1, 0);
                mem[addr]
            }
        }
    }

    /// Write the integer `val` to `addr`, silently growing memory if the addr
    /// hasn't been initialized yet.
    pub fn write(&mut self, addr: usize, val: isize) {
        let mem = self.mem.get_mut();
        match mem.get_mut(addr) {
            Some(v) => *v = val,
            None => {
                mem.resize(addr + 1, 0);
                mem[addr] = val;
            }
        }
    }
}
