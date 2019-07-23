use crate::vm::program::Action::{Move, Rotate, Fire};
use crate::vm::program::Direction::{Left, Right};
use crate::vm::program::Command::{Halt, LoadDirectA, LoadDirectB, LoadDirectAction};
use crate::vm::geom::Direction;
use crate::num_traits::{FromPrimitive, ToPrimitive};

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
#[derive(Primitive)]
pub enum Command {
    LoadDirectA = 0, // loads <arg> into a
    LoadDirectB = 1, // loads <arg> into b
    LoadDirectAction = 2, // loads
    Halt = 3, // halts program execution
    LogicNegateA = 4, // turns 0 into 1 and anything else into 0
    Add = 5, // adds a and b and puts into a
    SaveA = 6, // saves a into memory cell at <arg>
    LoadA = 7, // loads value at address <arg> into a
    LoadB = 8, // loads value at address <arg> into b
    SwapAB = 9, // swaps a and b,
    JumpA = 10, // sets instruction register to the value of a
}

pub type Program = Vec<u8>;