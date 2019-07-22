use std::fmt::Debug;
use crate::vm::geom::Coords;
use crate::vm::memory::Memory;

mod memory;
mod geom;


#[derive(Debug)]
pub struct VirtualMachine {
    players: Vec<String>,
    player_pos: Vec<Coords>,
    memory: Vec<Memory>
}

impl VirtualMachine {
    pub fn new(players: Vec<String>) -> Self {
        let size = players.len();

        VirtualMachine {
            players,
            player_pos: vec![Coords{x: 0, y: 0}; size],
            memory: vec![Memory::new(); size]
        }
    }
}