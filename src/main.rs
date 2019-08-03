extern crate tanks_vm;

mod server;

use tanks_vm::program::{Program};
use tanks_vm::VirtualMachine;
use tanks_vm::program::Command::*;
use tanks_vm::program::Action::*;

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

    println!("Hello, world! {:?}", v);

    let res = v.run();
    println!("game result: {:?}", res);
}
