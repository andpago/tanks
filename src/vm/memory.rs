use std::fmt::Debug;
use core::fmt;

// bot memory size in bytes
pub const MEMSIZE: usize = 128;


#[derive(Clone)]
pub struct Memory {
    memory: [i64; MEMSIZE]
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
        let v: Vec<i64> = self.memory.to_vec();
        v.fmt(f)
    }
}