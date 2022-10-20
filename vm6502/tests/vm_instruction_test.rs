use vm6502::prelude::*;
use vm6502::status;

// TODO: Backend to test direct instruction matching without
// modifying machine state. API decisions
#[test]
fn adc_imd() {
    let mut vm = VirtualMachine::new();
    let offset = 0x0000;
    vm.insert_program(offset, "69F06901");
    vm.registers.pc = offset as u16;
    vm.registers.ac = 0x0F;
    vm.tick();

    assert_eq!(vm.registers.ac, 0xFF);

    vm.tick();
    assert_eq!(vm.registers.sr & status!(Status::Carry), 1);
    assert_eq!(vm.registers.ac, 0x00);
}

#[test]
fn and_imd() {
    let mut vm = VirtualMachine::new();
    let offset = 0x0000;
    vm.insert_program(offset, "29FF2900");
    vm.registers.pc = offset as u16;
    vm.registers.ac = 0x0F;
    vm.tick();

    assert_eq!(vm.registers.ac, 0x0F);
    assert_eq!(vm.registers.sr & status!(Status::Zero), 0);

    vm.tick();
    assert_eq!(vm.registers.ac, 0x00);
    eprintln!("sr: 0x{:08b}", vm.registers.sr);
    assert_eq!(
        vm.registers.sr & status!(Status::Zero),
        status!(Status::Zero)
    );
}

#[test]
fn asl_simple() {
    let mut vm = VirtualMachine::new();
    let prog = "0A0A";
    vm.registers.ac = 0xFF;

    vm.insert_program(vm.vheap_bounds.0, prog);
    vm.tick();
    assert_eq!(vm.registers.ac, 0xFE);
    assert_eq!(vm.registers.sr & status!(Status::Carry), 1);

    vm.tick();
    assert_eq!(vm.registers.ac, 0xFC);
    assert_eq!(vm.registers.sr & status!(Status::Carry), 1);
}

#[test]
fn asl_cover() {
    let mut vm = VirtualMachine::new();
    let prog = "0A0A0A0A0A0A0A0A0A";
    vm.insert_program(vm.vheap_bounds.0, prog);
    vm.registers.ac = 0x01;

    for i in 1..8 {
        vm.tick();
        eprintln!("i: {}, ac: {}", 1 << i, vm.registers.ac);

        assert_eq!(vm.registers.ac, 1 << i);
    }
}

#[test]
fn bcc_simple() {
    let mut vm = VirtualMachine::new();
    let prog = "90F0";
    vm.insert_program(vm.vheap_bounds.0, prog);
    vm.registers.sr &= status!(Status::Carry);

    vm.tick();
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
        vm.insert_program(slide, prog.as_str());
        vm.tick();
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
        vm.insert_program(slide, prog);
        vm.tick();
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
        vm.insert_program(slide, prog);
        vm.tick();
        slide += 0xFE;
        eprintln!("|\tslide: 0x{:04X}, pc: 0x{:04X}", slide, vm.registers.pc);
        assert_eq!(vm.registers.pc, slide as u16);
    }
}
