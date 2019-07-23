use std::fmt::Debug;
use crate::vm::geom::Coords;
use crate::vm::memory::Memory;
use crate::vm::program::{Program, Action, Command};
use crate::vm::RuntimeErr::{PtrOutOfRange, InvalidAction, UnknownCommand, OutOfTime};

mod memory;
mod geom;
pub mod program;


const MAX_STEPS: i64 = 256;

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
    UnknownCommand,
    OutOfTime
}

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub instruction: u8,
    pub action: u8,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0,
            b: 0,
            instruction: 0,
            action: 0
        }
    }
}

fn execute_command(regs: &mut Registers, thing: Command, data: u8) -> bool {
    match thing {
        Command::LoadA => {
            println!("load a {}", data);
            regs.a = data;
        },
        Command::LoadB => {
            println!("load b {}", data);
            regs.b = data;
        },
        Command::Halt => {
            println!("halt");
            return true;
        },
        Command::LoadAction => {
            println!("load action {}", data);
            regs.action = data;
        }
    };
    false
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
        const INC: u8 = 2;

        let mut regs = Registers::new();
        let mut time = 0;

        let mem = &self.memory[idx];

        loop {
            time += 1;
            if time > MAX_STEPS {
                return Err(OutOfTime);
            }

            let cmd = match mem.get(regs.instruction as usize) {
                Some(x) => x,
                None => {return Err(PtrOutOfRange)}
            };
            regs.instruction += INC;

            if regs.instruction as usize >= mem.len() - 1 {
                return Err(PtrOutOfRange);
            }

            let data = match mem.get(regs.instruction as usize + 1) {
                Some(x) => x,
                None => {return Err(PtrOutOfRange)}
            };
            let command = Command::from_u8(&cmd);

            match command {
                None => {
                    return Err(UnknownCommand);
                },
                Some(thing) => {
                    let stop = execute_command(&mut regs, thing, data);
                    if stop {
                        break;
                    }
                },
            };
        }

        match Action::from_u8(regs.action) {
            Ok(action) => {
                Ok(action)
            }
            Err(_) => {
                Err(InvalidAction)
            }
        }
    }
}