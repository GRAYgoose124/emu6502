use hex::decode;

use vm6502::prelude::*;

#[test]
fn test_vm_stack() {
    let mut vm = VirtualMachine::new();

    let test_vec: arrayvec::ArrayVec<u8, 0x0FF> =
        (0..0x0FF).map(|_| rand::random::<u8>()).collect();
    let test: [u8; 0x0FF] = test_vec.into_inner().unwrap();

    // Push backwards because the stack grows from 0x01FF to 0x100.
    for i in 0x0FF..0 {
        vm.push(test[i]);
    }

    eprintln!("Stack: {:?}", vm);

    for i in 0x0FF..0 {
        assert_ne!(test[0x0FF - i - 1], 0);
        assert_ne!(vm.flatmap[0x0100 + i], 0);
        assert_eq!(vm.flatmap[0x0100 + i], test[0x0FF - i - 1]);
        vm.pop();
        assert_eq!(vm.registers.ac, test[0x0FF - i - 1]);
        assert_ne!(vm.registers.ac, 0);
    }
}

#[test]
fn test_vm_write_stack() {
    let mut vm = VirtualMachine::new();

    let test_vec: arrayvec::ArrayVec<u8, 0x0FF> =
        (0..0x0FF).map(|_| rand::random::<u8>()).collect();
    let test: [u8; 0x0FF] = test_vec.into_inner().unwrap();

    for i in 0..0x0FF {
        vm.push(test[i]);
    }

    // TODO: For some reason stack isn't being written properly.
    eprintln!("VM: {:?}", vm);
    eprintln!("Test: {:?}", test);

    for i in 0x0ff..=0 {
        vm.pop();
        if test[i] != 0 {
            assert_ne!(vm.registers.ac, 0);
        }
        assert_eq!(vm.registers.ac, test[i]);
    }
}

#[test]
fn test_vm_stack_contig() {
    let mut vm = VirtualMachine::new();

    let test_vec: arrayvec::ArrayVec<u8, 0x0FF> =
        (0..0x0FF).map(|_| rand::random::<u8>()).collect();
    let test: [u8; 0x0FF] = test_vec.into_inner().unwrap();

    // Push backwards because the stack grows from 0x01FF to 0x100.
    for i in 0..0x0FF {
        vm.push(test[i]);
    }

    eprintln!("vm {:?}", vm);
    eprintln!("stack=0x0100..0x01FF: {:?}", &vm.flatmap[0x100..0x01FF]);

    for (i, byte) in test.iter().rev().enumerate() {
        eprintln!("i: {}, byte: 0x{:02X}", i, byte);
        assert_eq!(vm.flatmap[vm.stack_bounds.0 + i + 1], *byte);
    }
}

#[test]
fn test_vm_registers() {
    let mut vm = VirtualMachine::new();
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
}

#[test]
fn test_vm_status() {
    let mut vm = VirtualMachine::new();
    vm.reset_status();
    assert_eq!(vm.registers.sr, 0x00);

    for i in 0..0x07 {
        let sv = 1 << i;

        vm.set_status(Status::from(sv), true);
        vm.flip_status(Status::from(sv));
        assert_eq!(vm.get_status(Status::from(sv)), false);
    }
}

#[test]
fn test_vm_insert_program() {
    let mut vm = VirtualMachine::new();
    let prog = "BADA55AB5214BADA55AB5214BADA55AB5214BADA55AB5214BADA55AB5214BADA55AB5214";
    let decoded = decode(prog).unwrap();

    let offset = 0x0000;
    vm.insert_program(offset, prog);

    for (i, byte) in decoded.iter().enumerate() {
        eprintln!("i: {}, byte: 0x{:02X}", i, byte);
        assert_eq!(
            vm.flatmap[(vm.heap_bounds.0 as u16 + offset + i as u16) as usize],
            *byte
        );
    }
}
