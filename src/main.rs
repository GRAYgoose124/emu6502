use vm6502::prelude::*;

fn main() {
    let mut vm = VirtM::new();
    vm.registers.ac = 0x01;
    vm.registers.sp = 0xFF;
    vm.flatmap[0x0000] = 0x01;

    for _ in 0..0x0FF {
        vm.registers.ac = rand::random::<u8>();
        vm.push();
    }

    for _ in 0..0x0FF {
        vm.pop();
    }

    vm.set_status(Status::Negative, true);

    let prog = "BADA55BADA55BADA55BADA55BADA55BADA55BADA55";
    vm.insert_program(vm.vheap_bounds.1 - (prog.len() / 2), prog);

    println!("{:?}", vm);
}
