#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

mod vm;
mod server;

use crate::vm::program::Action::*;
use crate::vm::program::Command::*;
use crate::vm::program::{Program, Action};
use vm::VirtualMachine;

fn main() {
    const HEAP: u8 = 255;
    const NONE: u8 = 0;


    let mut v = VirtualMachine::new(vec!["Hello".into(), "World".into()]);
    let p1: Program = vec![
        LoadA.u8(), HEAP,
        LoadDirectB.u8(), 5,
        Sub.u8(), NONE,
        LoadDirectB.u8(), 20,
        JumpBIfAPos.u8(), NONE,

        LoadA.u8(), HEAP,
        IncA.u8(), NONE,
        SaveA.u8(), HEAP,
        LoadDirectAction.u8(), Action::Move.into(),
        Halt.u8(), NONE,

        LoadDirectAction.u8(), Action::Fire.into(),
        Halt.u8(), NONE,
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
