use vm6502::prelude::*;

#[test]
fn test_status_macros() {
    use Status::*;

    assert_eq!(status!(Negative), 0b10000000);
    assert_eq!(status!(Overflow), 0b01000000);
    assert_eq!(status!(Unused), 0);
    assert_eq!(status!(Break), 0b00010000);
    assert_eq!(status!(Decimal), 0b00001000);
    assert_eq!(status!(Interrupt), 0b00000100);
    assert_eq!(status!(Zero), 0b00000010);
    assert_eq!(status!(Carry), 0b00000001);
}

#[test]
fn test_make_status() {
    use Status::*;

    let mut status = make_status!(Negative, Overflow, Break, Decimal, Interrupt, Zero, Carry);
    assert_eq!(status, 0b11011111);

    status = make_status!(Negative, Overflow, Break, Decimal, Interrupt, Zero);
    assert_eq!(status, 0b11011110);

    status = make_status!(Negative, Overflow, Break, Decimal, Interrupt);
    assert_eq!(status, 0b11011100);

    status = make_status!(Negative, Overflow, Break, Decimal);
    assert_eq!(status, 0b11011000);

    status = make_status!(Negative, Overflow, Break);
    assert_eq!(status, 0b11010000);

    status = make_status!(Negative, Overflow);
    assert_eq!(status, 0b11000000);

    status = make_status!(Negative);
    assert_eq!(status, 0b10000000);

    status = make_status!();
    assert_eq!(status, 0b00000000);
}

#[test]
fn test_stuff_program_at_end() {
    let mut vm = VirtM::new(); 

    let prog = "BADA55BADA55BADA55BADA55BADA55BADA55BADA55";
    stuff_program_at_end!(vm, prog);

    assert_eq!(vm.flatmap[vm.heap_bounds.1 - (prog.len() / 2)], 0xBA);
    println!("{:?}", vm);
}