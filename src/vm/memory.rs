use std::fmt::Debug;
use core::fmt;

// bot memory size in bytes
pub const MEMSIZE: usize = 16;

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
        if idx >= MEMSIZE {
            None
        } else {
            Some(self.memory[idx])
        }
    }

    pub fn set(self: &mut Self, data: &Vec<u8>) -> Result<(), ()> {
        if data.len() > MEMSIZE {
            return Err(())
        }

        for i in 0..data.len() {
            self.memory[i] = data[i];
        }

        Ok(())
    }
}

impl Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v: Vec<u8> = self.memory.to_vec();
        v.fmt(f)
    }
}