use std::fmt::Debug;
use crate::vm::geom::Coords;
use crate::vm::memory::Memory;
use crate::vm::program::{Program, Action, Command};

mod memory;
mod geom;
pub mod program;


#[derive(Debug)]
pub struct VirtualMachine {
    players: Vec<String>,
    player_pos: Vec<Coords>,
    memory: Vec<Memory>,
    programs: Vec<Program>,

    reg_a: u8,
    reg_b: u8,
    reg_cnt: u8, // pointer to the next command
}

impl VirtualMachine {
    pub fn new(players: Vec<String>) -> Self {
        let size = players.len();

        VirtualMachine {
            players,
            player_pos: vec![Coords{x: 0, y: 0}; size],
            memory: vec![Memory::new(); size],
            programs: vec![Program::new(); size],
            reg_a: 0,
            reg_b: 0,
            reg_cnt: 0,
        }
    }

    pub fn input(self: &mut Self, program: Program, player: String) -> Result<(), ()> {
        for i in 0..self.programs.len() {
            if self.players[i] == player {
                self.programs[i] = program;
                return Ok(())
            }
        }

        Err(())
    }

    pub fn turn(self: &Self, program: &Program) -> Option<Action> {
        let (mut reg_a, mut reg_b, mut reg_action) = (0, 0, 0);
        let mut cnt = 0;

        loop {
            let cmd = &program[cnt];
            let command = Command::from_cell(cmd);

            break
        }

        None
    }
}