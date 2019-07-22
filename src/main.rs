mod vm;

use vm::VirtualMachine;

fn main() {
    let v = VirtualMachine::new(vec!["Hello".into(), "World".into()]);
    println!("Hello, world! {:?}", v);
}
