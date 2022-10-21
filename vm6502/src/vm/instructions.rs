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
    fn adc(&mut self);
    fn and(&mut self);
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

        #[cfg(feature = "show_vm_instr")]
        println!(
            "\t\tADC: {:02X} + {:02X} = {:02X}, Carry: {:02X}",
            self.registers.ac,
            value,
            carried,
            status!(Status::Carry) & self.registers.sr
        );

        self.registers.ac = carried as u8;
        self.set_status(Status::Carry, carry);
    }

    fn and(&mut self) {
        let value = self.fetch();

        self.registers.ac &= value;

        #[cfg(feature = "show_vm_instr")]
        println!(
            "\t\tAND: {} & {} = {}",
            self.registers.ac, value, self.registers.ac
        );

        self.set_status(Status::Zero, self.registers.ac == 0);
    }

    fn asl(&mut self) {
        let value = self.fetch();

        let result = value << 1;
        let carry = result & 0x80 != 0;

        #[cfg(feature = "show_vm_instr")]
        println!("\t\tASL: {} << 1 = {}, carry: {}", value, result, carry);

        self.set_status(Status::Carry, carry);
        self.set_status(Status::Zero, result == 0);

        self.registers.ac = result;
    }

    fn bcc(&mut self, offset: u8) {
        #[cfg(feature = "show_vm_instr")]
        println!("\t\tBCC: 0x{:02X}", offset);

        self.relative_jump(offset, !self.get_status(Status::Carry));
    }

    fn bcs(&mut self, offset: u8) {
        #[cfg(feature = "show_vm_instr")]
        println!("\t\tBCS: 0x{:02X}", offset);

        self.relative_jump(offset, self.get_status(Status::Carry));
    }

    fn beq(&mut self, offset: u8) {
        #[cfg(feature = "show_vm_instr")]
        println!("\t\tBEQ: 0x{:02X}", offset);

        self.relative_jump(offset, self.get_status(Status::Zero));
    }

    fn bne(&mut self, offset: u8) {
        #[cfg(feature = "show_vm_instr")]
        println!("\t\tBEQ: 0x{:02X}", offset);

        self.relative_jump(offset, !self.get_status(Status::Zero));
    }

    fn bpl(&mut self, offset: u8) {
        #[cfg(feature = "show_vm_instr")]
        println!("\t\tBPL: 0x{:02X}", offset);

        self.relative_jump(offset, !self.get_status(Status::Negative));
    }

    fn bvc(&mut self, offset: u8) {
        #[cfg(feature = "show_vm_instr")]
        println!("\t\tBVC: 0x{:02X}", offset);

        self.relative_jump(offset, !self.get_status(Status::Overflow));
    }

    fn bvs(&mut self, offset: u8) {
        #[cfg(feature = "show_vm_instr")]
        println!("\t\tBVS: 0x{:02X}", offset);

        self.relative_jump(offset, self.get_status(Status::Overflow));
    }

    fn bit(&mut self) {
        // todo!();
    }

    fn bmi(&mut self, offset: u8) {
        #[cfg(feature = "show_vm_instr")]
        println!("\t\tBMI: 0x{:02X}", offset);

        self.relative_jump(offset, self.get_status(Status::Negative));
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

    fn dec(&mut self) {
        // todo!();
    }

    fn dex(&mut self) {
        // todo!();
    }

    fn dey(&mut self) {
        // todo!();
    }

    fn eor(&mut self) {
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

    fn jmp(&mut self) {
        // todo!();
    }

    fn jsr(&mut self) {
        // todo!();
    }

    fn lda(&mut self) {
        // todo!();
    }

    fn ldx(&mut self) {
        // todo!();
    }

    fn ldy(&mut self) {
        // todo!();
    }

    fn lsr(&mut self) {
        // todo!();
    }

    fn nop(&mut self) {}

    fn ora(&mut self) {
        let data = self.fetch();
        self.registers.ac |= data;

        self.set_status(Status::Zero, self.registers.ac == 0);
        self.set_status(Status::Negative, self.registers.ac & 0x80 != 0);
    }

    fn pha(&mut self) {
        // todo!();
    }

    fn php(&mut self) {
        // todo!();
    }

    fn pla(&mut self) {
        // todo!();
    }

    fn plp(&mut self) {
        // todo!();
    }

    fn rol(&mut self) {
        // todo!();
    }

    fn ror(&mut self) {
        // todo!();
    }

    fn rti(&mut self) {
        // todo!();
    }

    fn rts(&mut self) {
        // todo!();
    }

    fn sbc(&mut self) {
        // todo!();
    }

    fn sec(&mut self) {
        // todo!();
    }

    fn sed(&mut self) {
        // todo!();
    }

    fn sei(&mut self) {
        // todo!();
    }

    fn sta(&mut self) {
        // todo!();
    }

    fn stx(&mut self) {
        // todo!();
    }

    fn sty(&mut self) {
        // todo!();
    }

    fn tax(&mut self) {
        // todo!();
    }

    fn tay(&mut self) {
        // todo!();
    }

    fn tsx(&mut self) {
        // todo!();
    }

    fn txa(&mut self) {
        // todo!();
    }

    fn txs(&mut self) {
        // todo!();
    }

    fn tya(&mut self) {
        // todo!();
    }
}
