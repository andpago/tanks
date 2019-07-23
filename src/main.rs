#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

mod vm;

use crate::num_traits::ToPrimitive;
use crate::vm::program::Action::{Fire, Move, Rotate};
use crate::vm::program::Command::{Halt, JumpA, LoadDirectA, LoadDirectAction, LoadDirectB, Add, SaveA, LoadB, Sub, JumpBIfAPos, LoadA};
use crate::vm::program::Program;
use vm::VirtualMachine;
use crate::vm::geom::Direction;

fn main() {
    let mut v = VirtualMachine::new(vec!["Hello".into(), "World".into()]);
    let p1: Program = vec![
        LoadDirectAction.to_u8().unwrap(),
        Move.into(),
        Halt.to_u8().unwrap(),
    ];

    v.input(p1.clone(), "Hello".into()).unwrap();
    const t: u8 = 255;
    v.input(
        vec![
            LoadA.to_u8().unwrap(), t,
            LoadDirectB.to_u8().unwrap(), 1,
            Add.to_u8().unwrap(), 0,
            SaveA.to_u8().unwrap(), t,
            LoadDirectB.to_u8().unwrap(), 20,
            Sub.to_u8().unwrap(), 0,
            LoadDirectB.to_u8().unwrap(), 20,
            JumpBIfAPos.to_u8().unwrap(), 20,
            LoadDirectA.to_u8().unwrap(), 0,
            JumpA.to_u8().unwrap(), 6,
            LoadDirectAction.to_u8().unwrap(), Rotate(Direction::Left).into(),
            Halt.to_u8().unwrap()
        ],
        "World".into(),
    )
    .unwrap();

    println!("Hello, world! {:?}", v);

    v.run();
}
