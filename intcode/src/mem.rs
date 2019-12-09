use std::collections::HashMap;

use super::{Error, Result};

/// An Intcode machine memory module.
///
/// Uses a fixed-size Vec to store the base intcode program ("low memory"), with
/// a HashMap for any spill-over ("high memory")
#[derive(Debug, Clone)]
pub struct Mem {
    orig_mem: Vec<isize>,
    lo_mem: Vec<isize>,
    hi_mem: HashMap<usize, isize>,
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
            lo_mem: mem,
            hi_mem: HashMap::new(),
        })
    }

    /// Resets memory back to it's initial state
    pub fn reset(&mut self) {
        self.lo_mem.copy_from_slice(&self.orig_mem);
        self.hi_mem.clear();
    }

    /// Returns the length of the initial intcode program
    pub fn base_len(&self) -> usize {
        self.orig_mem.len()
    }

    /// Read the integer at `addr`, silently growing memory if the addr hasn't
    /// been initialized yet.
    pub fn read(&mut self, addr: usize) -> isize {
        match self.lo_mem.get(addr) {
            Some(v) => *v,
            None => *self.hi_mem.entry(addr).or_default(),
        }
    }

    /// Write the integer `val` to `addr`, silently growing memory if the addr
    /// hasn't been initialized yet.
    pub fn write(&mut self, addr: usize, val: isize) {
        match self.lo_mem.get_mut(addr) {
            Some(v) => *v = val,
            None => {
                self.hi_mem.insert(addr, val);
            }
        }
    }
}
