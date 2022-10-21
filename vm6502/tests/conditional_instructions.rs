use vm6502::prelude::*;
use vm6502::status;

#[test]
fn bcc_simple() {
    let mut vm = VirtualMachine::new();
    let prog = "90F0";
    vm.insert_program(vm.vheap_bounds.0 as u16, prog);
    vm.registers.sr &= status!(Status::Carry);

    vm.step();
    assert_eq!(vm.registers.pc, 0xF0);
}

#[test]
fn bcc_no_page_cross() {
    let mut vm = VirtualMachine::new();

    let offset = 0x03;
    let prog = format!("900{}", offset);
    let mut slide = vm.vheap_bounds.0;

    eprintln!(
        "Start slide:   0x{:04X}, pc: 0x{:04X}",
        slide, vm.registers.pc
    );
    assert_eq!(vm.registers.pc, slide as u16);

    for _ in 0..0x55 {
        vm.insert_program(slide as u16, prog.as_str());
        vm.step();
        slide += offset;
        eprintln!("slide: 0x{:04X}, pc: 0x{:04X}", slide, vm.registers.pc);
        assert_eq!(vm.registers.pc, slide as u16);
    }
}

#[test]
fn bcc_paging_cover_0xff() {
    let mut vm = VirtualMachine::new();
    let prog = "90FF";
    let mut slide = vm.vheap_bounds.0;

    eprintln!(
        "Start slide:   0x{:04X}, pc: 0x{:04X}",
        slide, vm.registers.pc
    );
    assert_eq!(vm.registers.pc, slide as u16);

    for i in vm.heap_bounds.0..=0xFF {
        eprintln!("|page 0x{:02X}", i);
        vm.insert_program(slide as u16, prog);
        vm.step();
        slide += 0xFF;
        eprintln!("|\tslide: 0x{:04X}, pc: 0x{:04X}", slide, vm.registers.pc);
        assert_eq!(vm.registers.pc, slide as u16);
    }
}

#[test]
fn bcc_paging_cover0xfe() {
    let mut vm = VirtualMachine::new();
    let prog = "90FE";
    let mut slide = vm.vheap_bounds.0;

    eprintln!(
        "Start slide:   0x{:04X}, pc: 0x{:04X}",
        slide, vm.registers.pc
    );
    assert_eq!(vm.registers.pc, slide as u16);

    for i in 0..=0xFF {
        eprintln!("|page 0x{:02X}", i);
        vm.insert_program(slide as u16, prog);
        vm.step();
        slide += 0xFE;
        eprintln!("|\tslide: 0x{:04X}, pc: 0x{:04X}", slide, vm.registers.pc);
        assert_eq!(vm.registers.pc, slide as u16);
    }
}

#[test]
fn bcs_paging_cover0xfe() {}
