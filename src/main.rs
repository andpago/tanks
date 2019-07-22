mod vm;

use vm::VirtualMachine;
use crate::vm::program::Program;
use crate::vm::program::Action::{Move, Rotate, Fire};
use crate::vm::program::Direction::Left;
use crate::vm::program::Command::{LoadA, LoadAction, Halt};

fn main() {
    let mut v = VirtualMachine::new(vec!["Hello".into(), "World".into()]);
    let p1: Program = vec![LoadAction.into(), Move.into(), Halt.into(), 0];

    v.input(p1, "Hello".into()).unwrap();

    println!("Hello, world! {:?}", v);
}
