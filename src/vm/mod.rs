use std::fmt::Debug;

pub const SIZE: usize = 20;

#[derive(Debug)]
#[derive(Clone)]
pub struct Coords {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug)]
pub struct VirtualMachine {
    players: Vec<String>,
    player_pos: Vec<Coords>
}

impl VirtualMachine {
    pub fn new(players: Vec<String>) -> Self {
        let size = players.len();

        VirtualMachine {
            players,
            player_pos: vec![Coords{x: 0, y: 0}; size]
        }
    }
}