use vm6502::prelude::*;

#[test]
fn test_status_macros() {
    assert_eq!(status!(Status::Negative), 0b10000000);
    assert_eq!(status!(Status::Overflow), 0b01000000);
    assert_eq!(status!(Status::Unused), 0);
    assert_eq!(status!(Status::Break), 0b00010000);
    assert_eq!(status!(Status::Decimal), 0b00001000);
    assert_eq!(status!(Status::Interrupt), 0b00000100);
    assert_eq!(status!(Status::Zero), 0b00000010);
    assert_eq!(status!(Status::Carry), 0b00000001);
}

#[test]
fn test_make_status() {
    use Status::*;
    let mut status = make_status!(Negative, Overflow, Break, Decimal, Interrupt, Zero, Carry);
    assert_eq!(status, 0b11011111);

    status = make_status!();
    assert_eq!(status, 0b00000000);
}
