use crate::num_traits::{FromPrimitive, ToPrimitive};
use crate::vm::geom::Coords;
use crate::vm::memory::Memory;
use crate::vm::program::{Action, Command, Program};
use crate::vm::tank::Tank;
use crate::vm::RuntimeErr::{InvalidAction, OutOfTime, PtrOutOfRange, UnknownCommand};
use std::fmt::Debug;

pub mod geom;
mod memory;
pub mod program;
mod tank;

const MAX_STEPS: i64 = 256;

#[derive(Debug)]
pub struct VirtualMachine {
    players: Vec<String>,
    tanks: Vec<Tank>,
    memory: Vec<Memory>,
    programs: Vec<Program>,
}

#[derive(Debug)]

pub enum RuntimeErr {
    PtrOutOfRange,
    InvalidAction,
    UnknownCommand(u8),
    OutOfTime,
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
            action: 0,
        }
    }
}

fn execute_command(
    regs: &mut Registers,
    thing: Command,
    data: u8,
    memory: &mut Memory,
) -> Result<bool, RuntimeErr> {
    match thing {
        Command::LoadDirectA => {
            regs.a = data;
        }
        Command::LoadDirectB => {
            regs.b = data;
        }
        Command::Halt => {
            return Ok(true);
        }
        Command::LoadDirectAction => {
            regs.action = data;
        }
        Command::LogicNegateA => {
            regs.a = match regs.a {
                0 => 1,
                _ => 0,
            }
        }
        Command::Add => regs.a = regs.a + regs.b,
        Command::SaveA => match memory.set_item(data as usize, regs.a) {
            Err(_) => return Err(PtrOutOfRange),
            Ok(_) => {}
        },
        Command::LoadA => match memory.get_item(data as usize) {
            Ok(saved) => {
                regs.a = saved;
            }
            Err(_) => return Err(PtrOutOfRange),
        },
        Command::LoadB => match memory.get_item(data as usize) {
            Ok(saved) => {
                regs.b = saved;
            }
            Err(_) => return Err(PtrOutOfRange),
        },
        Command::SwapAB => {
            let tmp = regs.b;
            regs.b = regs.a;
            regs.a = tmp;
        }
        Command::JumpA => {
            regs.instruction = regs.a;
        }
        Command::JumpBIfAPos => {
            if regs.a > 0 {
                regs.instruction = regs.b;
            }
        },
        Command::Sub => {
            if regs.a >= regs.b {
                regs.a -= regs.b;
            } else {
                regs.a = 0;
            }
        }
    };
    Ok(false)
}

impl VirtualMachine {
    pub fn new(players: Vec<String>) -> Self {
        let size = players.len();

        VirtualMachine {
            players,
            tanks: vec![Tank::new(); size],
            memory: vec![Memory::new(); size],
            programs: vec![Program::new(); size],
        }
    }

    fn exec_action(self: &mut Self, action: Action, player: usize) {
        match action {
            Action::Move => {
                println!(
                    "player {} moves from {:?} to the {:?}",
                    player, &self.tanks[player].pos, &self.tanks[player].dir
                );
                self.tanks[player].step();
            }
            Action::Rotate(rot) => {
                println!("player {} rotates to the {:?}", player, rot);
                self.tanks[player].dir = self.tanks[player].dir.rotate(&rot);
            }
            Action::Fire => {
                println!("player {} fires!", player);
            }
        }
    }

    pub fn input(self: &mut Self, program: Program, player: String) -> Result<(), &'static str> {
        for i in 0..self.programs.len() {
            if self.players[i] == player {
                self.memory[i].set(&program)?;
                self.programs[i] = program;
                return Ok(());
            }
        }

        Err("player not found")
    }

    pub fn decide_action(self: &mut Self, idx: usize) -> Result<Action, RuntimeErr> {
        const INC: u8 = 2;

        let mut regs = Registers::new();
        let mut time = 0;

        let mut mem = &mut self.memory[idx];

        loop {
            time += 1;
            if time > MAX_STEPS {
                return Err(OutOfTime);
            }

            let cmd = match mem.get_item(regs.instruction as usize) {
                Ok(x) => x,
                Err(_) => return Err(PtrOutOfRange),
            };

            let data = match mem.get_item(regs.instruction as usize + 1) {
                Ok(x) => x,
                Err(_) => return Err(PtrOutOfRange),
            };
            let command = Command::from_u8(cmd);

//            println!("{:?} {}, {:?}", command, data, regs);

            regs.instruction += INC;

            match command {
                None => {
                    return Err(UnknownCommand(cmd));
                }
                Some(thing) => {
                    let stop = execute_command(&mut regs, thing, data, mem)?;
                    if stop {
                        break;
                    }
                }
            };
        }

        match Action::from_u8(regs.action) {
            Ok(action) => Ok(action),
            Err(_) => Err(InvalidAction),
        }
    }

    pub fn run(self: &mut Self) {
        for _ in 0..10 {
            for player in 0..self.players.len() {
                let act = self.decide_action(player);
                match act {
                    Ok(action) => {
                        self.exec_action(action, player);
                    }
                    Err(e) => {
                        println!("player {} loses with {:?}", player, e);
                        return;
                        // TODO: this player loses
                    }
                }
            }
        }
    }
}
