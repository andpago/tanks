#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

mod vm;

use vm::VirtualMachine;
use crate::vm::program::Program;
use crate::vm::program::Action::{Move, Fire};
use crate::vm::program::Command::{LoadDirectAction, Halt, LoadDirectA, LoadDirectB, JumpA};
use crate::num_traits::{FromPrimitive, ToPrimitive};

fn main() {
    let mut v = VirtualMachine::new(vec!["Hello".into(), "World".into()]);
    let p1: Program = vec![LoadDirectAction.to_u8().unwrap(), Move.into(), Halt.to_u8().unwrap()];

    v.input(p1.clone(), "Hello".into()).unwrap();
    v.input(vec![
        LoadDirectA.to_u8().unwrap(), 0,
        LoadDirectB.to_u8().unwrap(), 0,
        JumpA.to_u8().unwrap()
    ], "World".into()).unwrap();

    println!("Hello, world! {:?}", v);

    v.run();
}
