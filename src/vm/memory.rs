use std::fmt::Debug;
use core::fmt;

// bot memory size in bytes
pub const MEMSIZE: usize = 128;
pub type Cell = u8;


#[derive(Clone)]
pub struct Memory {
    memory: [Cell; MEMSIZE]
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: [0; MEMSIZE]
        }
    }
}

impl Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v: Vec<Cell> = self.memory.to_vec();
        v.fmt(f)
    }
}