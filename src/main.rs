#![no_std]

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;
#[macro_use]
extern crate alloc;

#[macro_use]
extern crate cortex_m_semihosting;

mod vm;
mod server;

use crate::vm::program::{Program};
use vm::VirtualMachine;
use crate::vm::program::Command::*;
use crate::vm::program::Action::*;
use cortex_m_semihosting::{hprintln};

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
        LoadDirectAction.u8(), Move.into(),
        Halt.u8(), NONE,

        LoadDirectAction.u8(), Fire.into(),
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

    hprintln!("Hello, world! {:?}", v);

    let res = v.run();
    hprintln!("game result: {:?}", res);
}
