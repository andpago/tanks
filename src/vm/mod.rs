use crate::num_traits::{FromPrimitive, ToPrimitive};
use crate::vm::geom::{Coords, Direction};
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

fn point_in_ray(point: &Coords, start: &Coords, dir: &Direction) -> bool {
    match dir {
        Direction::Up => point.x == start.x && point.y > start.y,
        Direction::Right => point.y == start.y && point.x > start.x,
        Direction::Down => point.x == start.x && point.y < start.y,
        Direction::Left => point.y == start.y && point.x < start.x,
    }
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

    fn exec_action(self: &mut Self, action: &Action, player: usize) {
        const DAMAGE: i64 = 10;

        let pos = self.tanks[player].pos.clone();
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
                for tank in self.tanks.iter_mut() {
                    if point_in_ray(&pos, &tank.pos, &tank.dir) {
                        tank.hp -= DAMAGE;
                    }
                }
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

    pub fn run(self: &mut Self) -> Vec<WinStatus> {
        const MAX_TURNS: i64 = 10;

        for _ in 0..MAX_TURNS {
            let mut acts: Vec<Action> = Vec::new();

            for player in 0..self.players.len() {
                let act = self.decide_action(player);
                match act {
                    Ok(action) => {
                        acts.push(action);
                    }
                    Err(e) => {
                        println!("player {} loses with {:?}", player, e);
                        self.tanks[player].hp = 0;
                    }
                }
            }

            for (player, action) in acts.iter().enumerate() {
                if !self.tanks[player].alive() {
                    continue;
                }
                match action {
                    Action::Rotate(_)| Action::Move => {
                        self.exec_action(action, player)
                    }
                    _ => {}
                }
            }

            for (player, action) in acts.iter().enumerate() {
                if !self.tanks[player].alive() {
                    continue;
                }
                match action {
                    Action::Fire => {
                        self.exec_action(action, player)
                    }
                    _ => {}
                }
            }
        }

        let alive: usize = self.tanks.iter().map(|x|if x.alive(){1}else{0}).sum();
        let dead: usize = self.tanks.len() - alive;

        if alive == 0 || dead == 0 {
            return vec![WinStatus::Draw; self.tanks.len()];
        }

        if alive == 1 {
            return self.tanks.iter().map(|x|if x.alive() {WinStatus::Won} else {WinStatus::Lost}).collect();
        }

        self.tanks.iter().map(|x|if x.alive() {WinStatus::Draw} else {WinStatus::Lost}).collect()
    }
}

#[derive(Clone, Debug)]
pub enum WinStatus {
    Won,
    Lost,
    Draw
}
