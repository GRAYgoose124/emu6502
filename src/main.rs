/// Frontend for vm6502.
///
///
use vm6502::prelude::*;

fn main() {
    let mut vm = VirtualMachine::new();

    let prog = "69F00FBEEFDEADB00BBADA55";

    // vm.insert_program(vm.vheap_bounds.1 - (prog.len() / 2), prog);
    vm.insert_program(vm.vheap_bounds.0, prog);

    // let op = vm.g
    // vm.run_op()
    vm.fill_stack(prog.into());

    println!("{:?}", vm);
}
