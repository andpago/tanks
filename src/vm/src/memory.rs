use core::fmt;
use core::fmt::Debug;
use alloc::vec::Vec;

// bot memory size in bytes
pub const MEMSIZE: usize = 256;

#[derive(Clone)]
pub struct Memory {
    memory: [u8; MEMSIZE],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: [0; MEMSIZE],
        }
    }

    pub fn get_item(self: &Self, idx: usize) -> Result<u8, ()> {
        if idx >= MEMSIZE {
            Err(())
        } else {
            Ok(self.memory[idx])
        }
    }

    pub fn set(self: &mut Self, data: &Vec<u8>) -> Result<(), &'static str> {
        if data.len() > MEMSIZE {
            return Err("out of memory");
        }

        for i in 0..data.len() {
            self.memory[i] = data[i];
        }

        Ok(())
    }

    pub fn set_item(self: &mut Self, idx: usize, data: u8) -> Result<(), ()> {
        if idx >= MEMSIZE {
            return Err(());
        }

        self.memory[idx] = data;
        Ok(())
    }
}

impl Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v: Vec<u8> = self.memory.to_vec();
        v.fmt(f)
    }
}
