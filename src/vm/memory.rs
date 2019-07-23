use std::fmt::Debug;
use core::fmt;

// bot memory size in bytes
pub const MEMSIZE: usize = 128;

#[derive(Clone)]
pub struct Memory {
    memory: [u8; MEMSIZE]
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: [0; MEMSIZE]
        }
    }

    pub fn len(self: &Self) -> usize {
        self.memory.len()
    }

    pub fn get(self: &Self, idx: usize) -> Option<u8> {
        if idx < 0 || idx >= MEMSIZE {
            None
        } else {
            Some(self.memory[idx])
        }
    }
}

impl Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v: Vec<u8> = self.memory.to_vec();
        v.fmt(f)
    }
}