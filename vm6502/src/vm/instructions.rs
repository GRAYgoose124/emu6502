use crate::prelude::*;

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
    /// Bit test
    fn bit(&mut self);
    /// Break
    fn brk(&mut self);
    /// Clear carry flag
    fn clc(&mut self);
    /// Clear decimal mode
    fn cld(&mut self);
    /// Clear interrupt disable bit
    fn cli(&mut self);
    /// Clear overflow flag
    fn clv(&mut self);
    /// Compare
    fn cmp(&mut self);
    /// Compare X register
    fn cpx(&mut self);
    /// Compare Y register
    fn cpy(&mut self);
    /// Decrement memory
    fn dec(&mut self);
    /// Decrement X register
    fn dex(&mut self);
    /// Decrement Y register
    fn dey(&mut self);
    /// Exclusive OR
    fn eor(&mut self);
    /// Increment memory
    fn inc(&mut self);
    /// Increment X register
    fn inx(&mut self);
    /// Increment Y register
    fn iny(&mut self);
    /// Jump
    fn jmp(&mut self);
    /// Jump to subroutine
    fn jsr(&mut self);
    /// Load accumulator
    fn lda(&mut self);
    /// Load X register
    fn ldx(&mut self);
    /// Load Y register
    fn ldy(&mut self);
    /// Logical shift right
    fn lsr(&mut self);
    /// No operation
    fn nop(&mut self);
    /// Logical inclusive OR
    fn ora(&mut self);
    /// Push accumulator
    fn pha(&mut self);
    /// Push processor status (SR)
    fn php(&mut self);
    /// Pull accumulator
    fn pla(&mut self);
    /// Pull processor status (SR)
    fn plp(&mut self);
    /// Rotate left
    fn rol(&mut self);
    /// Rotate right
    fn ror(&mut self);
    /// Return from interrupt
    fn rti(&mut self);
    /// Return from subroutine
    fn rts(&mut self);
    /// Subtract with carry
    fn sbc(&mut self);
    /// Set carry flag
    fn sec(&mut self);
    /// Set decimal mode
    fn sed(&mut self);
    /// Set interrupt disable status
    fn sei(&mut self);
    /// Store accumulator
    fn sta(&mut self);
    /// Store X register
    fn stx(&mut self);
    /// Store Y register
    fn sty(&mut self);
    /// Transfer accumulator to X
    fn tax(&mut self);
    /// Transfer accumulator to Y
    fn tay(&mut self);
    /// Transfer stack pointer to X
    fn tsx(&mut self);
    /// Transfer X to accumulator
    fn txa(&mut self);
    /// Transfer X to stack pointer
    fn txs(&mut self);
    /// Transfer Y to accumulator
    fn tya(&mut self);
}

impl Instructions for VirtualMachine {
    fn brk(&mut self) {
        // Stop vm execution if we try incrementing from 0xFFFF
        // Not spec compliant.
        if self.registers.pc >= (u16::MAX - 1) {
            self.halted = true;
            return;
        }

        self.registers.pc = self.registers.pc.wrapping_add(1);

        self.push((self.registers.pc >> 8) as u8);
        self.push(self.registers.pc as u8);

        // Set the break flag inline, as it's not actually set in the status register.
        self.push(self.registers.sr | 0x10);
        self.set_status(Status::Interrupt, true);

        // Load the interrupt vector from 0xFFFE and 0xFFFF.
        // Subtracting heap offset to get the actual address.
        let jump = ((self.get_heap(0xFFFF - self.heap_bounds.0 as u16) as u16) << 8)
            | self.get_heap(0xFFFE - self.heap_bounds.0 as u16) as u16;

        self.registers.pc = jump;
    }

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

    // Incrementing OPs.
    fn dec(&mut self) {
        let addr = self.fetch();
        let operation = |value: u8| value.wrapping_sub(1);
        self.apply(addr as u16, operation);
    }

    fn inc(&mut self) {
        let addr = self.fetch();
        let operation = |value: u8| value.wrapping_add(1);
        self.apply(addr as u16, operation);
    }

    fn dex(&mut self) {
        self.registers.x = self.registers.x.wrapping_sub(1);
        self.set_status(Status::Zero, self.registers.x == 0);
        self.set_status(Status::Negative, self.registers.x & 0x80 != 0);
    }

    fn dey(&mut self) {
        self.registers.y = self.registers.y.wrapping_sub(1);
        self.set_status(Status::Zero, self.registers.y == 0);
        self.set_status(Status::Negative, self.registers.y & 0x80 != 0);
    }

    fn inx(&mut self) {
        self.registers.x = self.registers.x.wrapping_add(1);
        self.set_status(Status::Zero, self.registers.x == 0);
        self.set_status(Status::Negative, self.registers.x & 0x80 != 0);
    }

    fn iny(&mut self) {
        self.registers.y = self.registers.y.wrapping_add(1);
        self.set_status(Status::Zero, self.registers.y == 0);
        self.set_status(Status::Negative, self.registers.y & 0x80 != 0);
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

    // Comparison OPs
    fn bit(&mut self) {
        let value = self.fetch();
        let result = self.registers.ac & value;

        self.set_status(Status::Zero, result == 0);
        self.set_status(Status::Negative, value & 0x80 != 0);
        self.set_status(Status::Overflow, value & 0x40 != 0);
    }

    fn cmp(&mut self) {
        let value = self.fetch();
        let result = self.registers.ac as u16 - value as u16;

        self.set_status(Status::Carry, result < 0x100);
        self.set_status(Status::Zero, result == 0);
        self.set_status(Status::Negative, result & 0x80 != 0);
    }

    fn cpx(&mut self) {
        let value = self.fetch();
        let result = self.registers.x as u16 - value as u16;

        self.set_status(Status::Carry, result < 0x100);
        self.set_status(Status::Zero, result == 0);
        self.set_status(Status::Negative, result & 0x80 != 0);
    }

    fn cpy(&mut self) {
        let value = self.fetch();
        let result = self.registers.y as u16 - value as u16;

        self.set_status(Status::Carry, result < 0x100);
        self.set_status(Status::Zero, result == 0);
        self.set_status(Status::Negative, result & 0x80 != 0);
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
        let haddr = self.fetch();
        let laddr = self.fetch();

        self.registers.pc = (haddr as u16) << 8 | laddr as u16;
    }

    fn jsr(&mut self) {
        let haddr = self.fetch();
        let laddr = self.fetch();

        let pc = self.registers.pc - 1;
        self.push((pc >> 8) as u8);
        self.push(pc as u8);

        self.registers.pc = (haddr as u16) << 8 | laddr as u16;
    }

    fn rti(&mut self) {
        let sts = self.pop();
        // Pull SR and ignore BRK and bit 5.
        self.registers.sr = sts & 0b0011_1111;
        // Pull PC
        self.registers.pc = self.pop() as u16;
        self.registers.pc |= (self.pop() as u16) << 8;
    }

    fn rts(&mut self) {
        // Pull PC from stack.
        let addr = self.pop() as u16;
        let addr = addr | (self.pop() as u16) << 8;

        self.registers.pc = addr + 1;
    }

    // Flag set OPs
    fn clc(&mut self) {
        self.set_status(Status::Carry, false);
    }

    fn cld(&mut self) {
        self.set_status(Status::Decimal, false);
    }

    fn cli(&mut self) {
        self.set_status(Status::Interrupt, false);
    }

    fn clv(&mut self) {
        self.set_status(Status::Overflow, false);
    }

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
        // I've made some glaring assumptions about the 6502 that are catching up now.
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
