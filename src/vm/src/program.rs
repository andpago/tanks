use crate::num_traits::{ToPrimitive};
use crate::geom::Direction;
use crate::program::Action::{Fire, Move, Rotate};
use alloc::vec::Vec;

// Actions will be placed into the action register
#[derive(Debug)]
pub enum Action {
    Move,
    Rotate(Direction),
    Fire,
}

impl Into<u8> for Action {
    fn into(self: Self) -> u8 {
        match self {
            Move => 1,
            Action::Rotate(dir) => dir as u8 + 2,
            Fire => 6,
        }
    }
}

impl Action {
    pub fn from_u8(bt: u8) -> Result<Self, ()> {
        match bt {
            1 => Ok(Move),
           2..=5 => Ok(Rotate(Direction::from(bt - 2))),
            6 => Ok(Fire),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Primitive)]
pub enum Command {
    LoadDirectA = 0,      // loads <arg> into A
    LoadDirectB = 1,      // loads <arg> into B
    LoadDirectAction = 2, // loads <arg> into action register
    Halt = 3,             // halts program execution
    LogicNegateA = 4,     // turns 0 into 1 and anything else into 0
    Add = 5,              // adds A and B and puts result into A
    SaveA = 6,            // saves A into memory cell at <arg>
    LoadA = 7,            // loads value at address <arg> into A
    LoadB = 8,            // loads value at address <arg> into B
    SwapAB = 9,           // swaps A and B,
    JumpA = 10,           // sets instruction register to the value of A
    JumpBIfAPos = 11, // jump to <arg> if A is not zero
    Sub = 12, // subtract B from A,
    IncA = 13, // increment A register
    DecA = 14 // decrement A register
}

impl Command {
    pub fn u8(self: &Self) -> u8 {
        self.to_u8().unwrap()
    }
}

pub type Program = Vec<u8>;
