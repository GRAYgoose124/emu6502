use crate::prelude::*;
#[cfg(feature = "show_status")]
use crate::status;

pub mod prelude {
    pub use crate::vm::instructions::Instructions;
}

/// Adds the instructions to the vm.
///
/// This is placed in a separate trait due to the inherent number of instructions.
pub trait Instructions {
    fn abs_zp_acc_op(&mut self, operation: fn(u8) -> u8) -> u8;
    /// Add with carry
    fn adc(&mut self);
    /// Logical AND
    fn and(&mut self);
    /// Arithmetic shift left
    fn asl(&mut self);

    // Conditional instructions
    /// Branch on carry clear
    fn bcc(&mut self, offset: u8);
    /// Branch on carry set
    fn bcs(&mut self, offset: u8);
    /// Branch on equal (zero set)
    fn beq(&mut self, offset: u8);
    /// Branch on minus (negative set)
    fn bne(&mut self, offset: u8);
    /// Branch on plus (negative clear)
    fn bpl(&mut self, offset: u8);
    // Branch on Minus (negative set)
    fn bmi(&mut self, offset: u8);
    /// Branch on overflow clear
    fn bvc(&mut self, offset: u8);
    /// Branch on overflow set
    fn bvs(&mut self, offset: u8);

    fn bit(&mut self);
    fn brk(&mut self);
    fn clc(&mut self);
    fn cld(&mut self);
    fn cli(&mut self);
    fn clv(&mut self);
    fn cmp(&mut self);
    fn cpx(&mut self);
    fn cpy(&mut self);
    fn dec(&mut self);
    fn dex(&mut self);
    fn dey(&mut self);
    fn eor(&mut self);
    fn inc(&mut self);
    fn inx(&mut self);
    fn iny(&mut self);
    fn jmp(&mut self);
    fn jsr(&mut self);
    fn lda(&mut self);
    fn ldx(&mut self);
    fn ldy(&mut self);
    fn lsr(&mut self);
    fn nop(&mut self);
    fn ora(&mut self);
    fn pha(&mut self);
    fn php(&mut self);
    fn pla(&mut self);
    fn plp(&mut self);
    fn rol(&mut self);
    fn ror(&mut self);
    fn rti(&mut self);
    fn rts(&mut self);
    fn sbc(&mut self);
    fn sec(&mut self);
    fn sed(&mut self);
    fn sei(&mut self);
    fn sta(&mut self);
    fn stx(&mut self);
    fn sty(&mut self);
    fn tax(&mut self);
    fn tay(&mut self);
    fn tsx(&mut self);
    fn txa(&mut self);
    fn txs(&mut self);
    fn tya(&mut self);
}

impl Instructions for VirtualMachine {
    fn adc(&mut self) {
        let value = self.fetch(); // Fetch is directed by the internal mode.

        let result = self.registers.ac as u16 + value as u16;
        let (carry, carried) = (result > 0xFF, result & 0xFF);

        self.registers.ac = carried as u8;
        self.set_status(Status::Carry, carry);
    }

    fn sbc(&mut self) {
        let value = self.fetch(); // Fetch is directed by the internal mode.

        let result = self.registers.ac as u16 - value as u16;
        let (carry, carried) = (result > 0xFF, result & 0xFF);

        self.registers.ac = carried as u8;
        self.set_status(Status::Carry, carry);
    }

    fn and(&mut self) {
        let value = self.fetch();
        self.registers.ac &= value;

        self.set_status(Status::Zero, self.registers.ac == 0);
    }

    fn eor(&mut self) {
        let value = self.fetch();
        self.registers.ac ^= value;

        self.set_status(Status::Zero, self.registers.ac == 0);
    }

    fn ora(&mut self) {
        let data = self.fetch();
        self.registers.ac |= data;

        self.set_status(Status::Zero, self.registers.ac == 0);
        self.set_status(Status::Negative, self.registers.ac & 0x80 != 0);
    }

    fn bcc(&mut self, offset: u8) {
        self.relative_jump(offset, !self.get_status(Status::Carry));
    }

    fn bcs(&mut self, offset: u8) {
        self.relative_jump(offset, self.get_status(Status::Carry));
    }

    fn beq(&mut self, offset: u8) {
        self.relative_jump(offset, self.get_status(Status::Zero));
    }

    fn bne(&mut self, offset: u8) {
        self.relative_jump(offset, !self.get_status(Status::Zero));
    }

    fn bpl(&mut self, offset: u8) {
        self.relative_jump(offset, !self.get_status(Status::Negative));
    }

    fn bvc(&mut self, offset: u8) {
        self.relative_jump(offset, !self.get_status(Status::Overflow));
    }

    fn bvs(&mut self, offset: u8) {
        self.relative_jump(offset, self.get_status(Status::Overflow));
    }

    fn bmi(&mut self, offset: u8) {
        self.relative_jump(offset, self.get_status(Status::Negative));
    }

    fn bit(&mut self) {
        // todo!();
    }

    fn brk(&mut self) {}

    fn clc(&mut self) {
        // todo!();
    }

    fn cld(&mut self) {
        // todo!();
    }

    fn cli(&mut self) {
        // todo!();
    }

    fn clv(&mut self) {
        // todo!();
    }

    fn cmp(&mut self) {
        // todo!();
    }

    fn cpx(&mut self) {
        // todo!();
    }

    fn cpy(&mut self) {
        // todo!();
    }

    // Incrementing OPs.
    fn dec(&mut self) {}

    fn dex(&mut self) {}

    fn dey(&mut self) {
        // todo!();
    }
    fn inc(&mut self) {
        // todo!();
    }

    fn inx(&mut self) {
        // todo!();
    }

    fn iny(&mut self) {
        // todo!();
    }

    /// Load Instructions;
    fn lda(&mut self) {
        let data = self.fetch();
        self.registers.ac = data;

        self.set_status(Status::Zero, data == 0);
        self.set_status(Status::Negative, data & 0x80 != 0);
    }

    fn ldx(&mut self) {
        let data = self.fetch();
        self.registers.x = data;

        self.set_status(Status::Zero, data == 0);
        self.set_status(Status::Negative, data & 0x80 != 0);
    }

    fn ldy(&mut self) {
        let data = self.fetch();
        self.registers.y = data;

        self.set_status(Status::Zero, data == 0);
        self.set_status(Status::Negative, data & 0x80 != 0);
    }

    // TODO: Move to separate mod, general_instructions?
    /// Accumulator <-> ZeroPage, Absolute, ZeroPageX and AbsoluteX
    fn abs_zp_acc_op(&mut self, operation: fn(u8) -> u8) -> u8 {
        let data = self.fetch();

        // Performs an operation and moves it to destination.
        let result = self.apply(data as u16, operation);

        self.set_status(Status::Carry, result & 0x01 != 0);
        self.set_status(Status::Zero, result == 0);
        self.set_status(Status::Negative, result & 0x80 != 0);

        data
    }

    fn lsr(&mut self) {
        self.abs_zp_acc_op(|d| d >> 1);
    }

    fn asl(&mut self) {
        self.abs_zp_acc_op(|d| d << 1);
    }

    fn rol(&mut self) {
        let data = self.abs_zp_acc_op(|d| d << 1);
        if self.get_status(Status::Carry) {
            self.apply(data as u16, |d| d | 0x01);
        };
    }

    fn ror(&mut self) {
        let data = self.abs_zp_acc_op(|d| d >> 1);
        let sts = self.get_status(Status::Carry);
        if sts {
            match self.addr_mode {
                Mode::Accumulator => self.registers.ac |= 0x80,
                Mode::ZeroPage => self.flatmap[data as usize] |= 0x80,
                Mode::ZeroPageX => self.flatmap[(data + self.registers.x) as usize] |= 0x80,
                Mode::Absolute => self.flatmap[data as usize] |= 0x80,
                Mode::AbsoluteX => self.flatmap[(data + self.registers.x) as usize] |= 0x80,

                _ => panic!("Invalid addressing mode for ROL"),
            }
        };
    }

    // Jumping/Procedure OPs
    fn jmp(&mut self) {
        //let addr = self.fetch();
    }

    fn jsr(&mut self) {
        // todo!();
    }

    fn rti(&mut self) {
        // todo!();
    }

    fn rts(&mut self) {}

    // Flag set OPs
    fn sec(&mut self) {
        self.set_status(Status::Carry, true);
    }

    fn sed(&mut self) {
        self.set_status(Status::Decimal, true);
    }

    fn sei(&mut self) {
        // Technically interrupt disable.
        self.set_status(Status::Interrupt, true);
    }

    // Store Operations
    fn sta(&mut self) {
        let data = self.fetch();
        // TODO: This code duplication can likely be refactored, similarly to ROL/ROR, but more general?
        match self.addr_mode {
            Mode::ZeroPage => self.flatmap[data as usize] = self.registers.ac,
            Mode::ZeroPageX => self.flatmap[(data + self.registers.x) as usize] = self.registers.ac,
            Mode::Absolute => self.flatmap[data as usize] = self.registers.ac,
            Mode::AbsoluteX => self.flatmap[(data + self.registers.x) as usize] = self.registers.ac,
            Mode::AbsoluteY => self.flatmap[(data + self.registers.y) as usize] = self.registers.ac,
            Mode::IndirectX => self.flatmap[(data + self.registers.x) as usize] = self.registers.ac,
            Mode::IndirectY => self.flatmap[(data + self.registers.y) as usize] = self.registers.ac,
            _ => panic!("Invalid addressing mode for STA"),
        }
    }

    fn stx(&mut self) {
        let data = self.fetch();
        match self.addr_mode {
            Mode::ZeroPage => self.flatmap[data as usize] = self.registers.x,
            Mode::ZeroPageY => self.flatmap[(data + self.registers.y) as usize] = self.registers.x,
            Mode::Absolute => self.flatmap[data as usize] = self.registers.x,
            _ => panic!("Invalid addressing mode for STX"),
        }
    }

    fn sty(&mut self) {
        let data = self.fetch();
        match self.addr_mode {
            Mode::ZeroPage => self.flatmap[data as usize] = self.registers.y,
            Mode::ZeroPageX => self.flatmap[(data + self.registers.x) as usize] = self.registers.y,
            Mode::Absolute => self.flatmap[data as usize] = self.registers.y,
            _ => panic!("Invalid addressing mode for STY"),
        }
    }

    // Transfer register Ops.
    fn tax(&mut self) {
        self.registers.x = self.registers.ac;
        // TODO: We can definitely create a helper for the transfer instructions.
        self.set_status(Status::Zero, self.registers.x == 0);
        self.set_status(Status::Negative, self.registers.x & 0x80 != 0);
    }

    fn txa(&mut self) {
        self.registers.ac = self.registers.x;
        self.set_status(Status::Zero, self.registers.ac == 0);
        self.set_status(Status::Negative, self.registers.ac & 0x80 != 0);
    }

    fn tay(&mut self) {
        self.registers.y = self.registers.ac;
        self.set_status(Status::Zero, self.registers.y == 0);
        self.set_status(Status::Negative, self.registers.y & 0x80 != 0);
    }

    fn tya(&mut self) {
        self.registers.ac = self.registers.y;
        self.set_status(Status::Zero, self.registers.ac == 0);
        self.set_status(Status::Negative, self.registers.ac & 0x80 != 0);
    }

    fn tsx(&mut self) {
        self.registers.x = self.registers.sp;
        self.set_status(Status::Zero, self.registers.x == 0);
        self.set_status(Status::Negative, self.registers.x & 0x80 != 0);
    }

    fn txs(&mut self) {
        self.registers.sp = self.registers.x;
    }

    // Register Push/Pull Ops
    fn pha(&mut self) {
        self.push(self.registers.ac);
    }

    fn php(&mut self) {
        self.push(self.registers.sr);
    }

    fn pla(&mut self) {
        let sts = self.pop();
        self.registers.ac = sts;

        self.set_status(Status::Zero, sts == 0);
        self.set_status(Status::Negative, sts & 0x80 != 0);
    }

    fn plp(&mut self) {
        let sts = self.pop();
        self.registers.sr = sts;
    }

    fn nop(&mut self) {}
}
