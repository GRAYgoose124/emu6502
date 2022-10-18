use rand::prelude::*;

use vm6502::vm::VirtM;
use vm6502::vm::StackInterface;

fn main() {
    let mut vm = VirtM::new();
    vm.registers.ac = 0x01;
    vm.registers.sp = 0xFF;
    vm.flatmap[0x0000] = 0x01;
    
    for _ in 0..0x0FF {
        vm.registers.ac = rand::random::<u8>();
        vm.push();
    }

    println!("{:?}", vm);
}

