use arrayvec::ArrayVec;

use vm6502::prelude::*;

fn main() {
    let mut vm = VirtualMachine::new();
    vm.registers.ac = 0x01;
    vm.registers.sp = 0xFF;
    vm.flatmap[0x0000] = 0x01;

    let test_vec: ArrayVec<u8, 0x0FF> = (0..0x0FF).map(|_| rand::random::<u8>()).collect();
    let test: [u8; 0x0FF] = test_vec.into_inner().unwrap();

    for item in &test {
        vm.registers.ac = *item;
        vm.push();
    }

    // TODO: For some reason stack isn't being written properly.
    // It's working in the tests though.
    println!("VM: {:?}", vm);

    for _ in 0..0x0FF {
        vm.pop();
    }

    vm.set_status(Status::Negative, true);

    let prog = "BADA55BADA55BADA55BADA55BADA55BADA55BADA55";
    vm.insert_program(vm.vheap_bounds.1 - (prog.len() / 2), prog);

    println!("{:?}", vm);
}
