use crate::vm::program::Action::{Move, Rotate, Fire};
use crate::vm::program::Direction::{Left, Right};
use crate::vm::program::Command::{Halt, LoadA, LoadB, LoadAction};
use crate::vm::geom::Direction;

// Actions will be placed into the action register
#[derive(Debug)]
pub enum Action {
    Move,
    Rotate(Direction),
    Fire
}

impl Into<u8> for Action {
    fn into(self: Self) -> u8 {
        match self {
            Move => 1,
            Action::Rotate(dir) => {
                dir as u8 + 1
            }
            Fire => 4,
        }
    }
}

impl Action {
    pub fn from_u8(bt: u8) -> Result<Self, ()> {
        match bt {
            1 => Ok(Move),
            2 => Ok(Rotate(Left)),
            3 => Ok(Rotate(Right)),
            4 => Ok(Fire),
            _ => Err(())
        }
    }
}

#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum Command {
    LoadA = 0,
    LoadB = 1,
    LoadAction = 2,
    Halt = 3
}

impl Into<u8> for Command {
    fn into(self: Self) -> u8 {
        self as u8
    }
}

impl Command {
    pub fn from_u8(bt: &u8) -> Option<Command> {
        match bt {
            0 => Some(LoadA),
            1 => Some(LoadB),
            2 => Some(LoadAction),
            3 => Some(Halt),
            _ => None
        }
    }
}

pub type Program = Vec<u8>;