#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

mod vm;

use crate::num_traits::ToPrimitive;
use crate::vm::program::Action::{Fire, Move, Rotate};
use crate::vm::program::Command::{Halt, JumpA, LoadDirectA, LoadDirectAction, LoadDirectB, Add, SaveA, LoadB, Sub, JumpBIfAPos, LoadA, IncA, LogicNegateA};
use crate::vm::program::{Program, Action};
use vm::VirtualMachine;
use crate::vm::geom::Direction;

fn main() {
    const heap: u8 = 255;
    const none: u8 = 0;


    let mut v = VirtualMachine::new(vec!["Hello".into(), "World".into()]);
    let p1: Program = vec![
        LoadA.u8(), heap,
        LoadDirectB.u8(), 5,
        Sub.u8(), none,
        LoadDirectB.u8(), 20,
        JumpBIfAPos.u8(), none,

        LoadA.u8(), heap,
        IncA.u8(), none,
        SaveA.u8(), heap,
        LoadDirectAction.u8(), Action::Move.into(),
        Halt.u8(), none,

        LoadDirectAction.u8(), Action::Fire.into(),
        Halt.u8(), none,
    ];

    v.input(p1.clone(), "Hello".into()).unwrap();
    v.input(
        vec![
            LoadDirectAction.u8(), Move.into(),
            Halt.u8()
        ],
        "World".into(),
    )
    .unwrap();

    println!("Hello, world! {:?}", v);

    let res = v.run();
    println!("game result: {:?}", res);
}
