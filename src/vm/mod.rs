use std::fmt::Debug;
use crate::vm::geom::Coords;
use crate::vm::memory::Memory;
use crate::vm::program::{Program, Action, Command};
use crate::vm::RuntimeErr::{PtrOutOfRange, InvalidAction, UnknownCommand};

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

#[derive(Debug)]
pub enum RuntimeErr {
    PtrOutOfRange,
    InvalidAction,
    UnknownCommand
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

    pub fn turn(self: &mut Self, idx: usize) -> Result<Action, RuntimeErr> {
        let mut reg_a = 0;
        let mut reg_b = 0;
        let mut reg_action = 0;

        let mut cnt = 0;

        let mem = &self.memory[idx];

        loop {
            if cnt >= mem.len() - 1 {
                return Err(PtrOutOfRange);
            }

            let cmd = match mem.get(cnt) {Some(x) => x, None => {return Err(PtrOutOfRange)}};
            let data = match mem.get(cnt + 1) {Some(x) => x, None => {return Err(PtrOutOfRange)}};
            let command = Command::from_u8(&cmd);
            let mut next: Option<Action> = None;

            match command {
                None => {
                    return Err(UnknownCommand);
                },
                Some(thing) => {
                    match thing {
                        LoadA => {
                            reg_a = data;
                        },
                        LoadB => {
                            reg_b = data;
                        },
                        Halt => {
                            // return
                        },
                        LoadAction => {
                            let action = Action::from_u8(data);
                            match action {
                                Ok(ac) => {
                                    next = Some(ac);
                                },
                                Err(_) => {
                                    return Err(InvalidAction)
                                }
                            }
                        }
                    };
                },
            }

            cnt += 2;
        }

        match Action::from_u8(reg_action) {
            Ok(action) => {
                Ok(action)
            }
            Err(_) => {
                Err(InvalidAction)
            }
        }
    }
}