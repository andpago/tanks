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
                self.memory[i].set(&program)?;
                self.programs[i] = program;
                return Ok(())
            }
        }

        Err(())
    }

    pub fn turn(self: &mut Self, idx: usize) -> Result<Action, RuntimeErr> {
        const INC: usize = 2;

        let mut reg_a = 0;
        let mut reg_b = 0;
        let mut reg_action = 0;

        let mut cnt = 0;

        let mem = &self.memory[idx];

        loop {
            let cmd = match mem.get(cnt) {Some(x) => x, None => {return Err(PtrOutOfRange)}};
            cnt += INC;

            if cnt >= mem.len() - 1 {
                return Err(PtrOutOfRange);
            }

            let data = match mem.get(cnt + 1) {Some(x) => x, None => {return Err(PtrOutOfRange)}};
            let command = Command::from_u8(&cmd);
            let mut next: Option<u8> = None;

            match command {
                None => {
                    return Err(UnknownCommand);
                },
                Some(thing) => {
                    match thing {
                        Command::LoadA => {
                            println!("load a {}", data);
                            reg_a = data;
                        },
                        Command::LoadB => {
                            println!("load b {}", data);
                            reg_b = data;
                        },
                        Command::Halt => {
                            println!("halt");
                            break;
                        },
                        Command::LoadAction => {
                            println!("load action {}", data);
                            next = Some(data);
                        }
                    };
                },
            };

            match next {
                Some(a) => {
                    reg_action = a;
                },
                None => {}
            }
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