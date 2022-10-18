use hex::decode;

use vm6502::prelude::*;

#[test]
fn test_vm_stack() {
    let mut vm = VirtM::new();

    let test: [u8; 0x0FF] = [rand::random::<u8>(); 0x0FF];
    for i in 0x0FF..0 {
        vm.registers.ac = test[i];
        vm.push();
    }

    for i in 0x0FF..0 {
        vm.pop();
        assert_eq!(vm.registers.ac, test[0x0FF - i - 1]);
    }


    println!("{:?}", vm);
}

#[test]
fn test_vm_registers() {
    let mut vm = VirtM::new();
    vm.registers.ac = 0x01;
    vm.registers.x = 0x02;
    vm.registers.y = 0x03;
    vm.registers.sr = 0x04;
    vm.registers.sp = 0x05;
    vm.registers.pc = 0x06;

    assert_eq!(vm.registers.ac, 0x01);
    assert_eq!(vm.registers.x, 0x02);
    assert_eq!(vm.registers.y, 0x03);
    assert_eq!(vm.registers.sr, 0x04);
    assert_eq!(vm.registers.sp, 0x05);
    assert_eq!(vm.registers.pc, 0x06);
    println!("{:?}", vm);
}

#[test]
fn test_vm_status() {
    let mut vm = VirtM::new();
    vm.reset_status();
    assert_eq!(vm.registers.sr, 0x00);
    
    for i in 0..0x08 {
        vm.set_status(Status::from(i), true);
        vm.flip_status(Status::from(i));
        assert_eq!(vm.get_status(Status::from(i)), false);
    }

    println!("{:?}", vm);
}

#[test]
fn test_vm_insert_program(){
    let mut vm = VirtM::new();
    let prog = "BADA55AB5214BADA55AB5214BADA55AB5214BADA55AB5214BADA55AB5214BADA55AB5214";
    let decoded = decode(prog).unwrap();

    let offset = 0x0000;
    vm.insert_program(offset, prog);

    for (i, byte) in decoded.iter().enumerate() {
        assert_eq!(vm.flatmap[vm.heap_bounds.0 + offset + i], *byte);
    }
}