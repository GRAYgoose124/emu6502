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

    fn bmi(&mut self, offset: u8) {
        #[cfg(feature = "show_vm_instr")]
        println!("\t\tBMI: 0x{:02X}", offset);

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
        //let addr = self.fetch();
    }

    fn jsr(&mut self) {
        // todo!();
    }

    fn lda(&mut self) {
        let data = self.fetch();
        self.registers.ac = data;

        self.set_status(Status::Zero, self.registers.ac == 0);
        self.set_status(Status::Negative, self.registers.ac & 0x80 != 0);
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

    fn lsr(&mut self) {
        let data = self.fetch();

        // If there's a 1 in the 1's place, it will shifted off.
        let carried = |f| f & 0x01 != 0;

        // We gotta be able to refactor out all these matches.
        let (c, r) = match self.addr_mode {
            Mode::Accumulator => {
                let carry = carried(self.registers.ac);
                let result = self.registers.ac >> 1;
                self.registers.ac = result;
                (carry, result)
            }
            Mode::ZeroPage => {
                let carry = carried(self.flatmap[data as usize]);
                let result = self.flatmap[data as usize] >> 1;
                self.flatmap[data as usize] = result;
                (carry, result)
            }
            Mode::ZeroPageX => {
                let carry = carried(self.flatmap[(data + self.registers.x) as usize]);
                let result = self.flatmap[(data + self.registers.x) as usize] >> 1;
                self.flatmap[(data + self.registers.x) as usize] = result;
                (carry, result)
            }
            Mode::Absolute => {
                let carry = carried(self.flatmap[data as usize]);
                let result = self.flatmap[data as usize] >> 1;
                self.flatmap[data as usize] = result;
                (carry, result)
            }
            Mode::AbsoluteX => {
                let carry = carried(self.flatmap[(data + self.registers.x) as usize]);
                let result = self.flatmap[(data + self.registers.x) as usize] >> 1;
                self.flatmap[(data + self.registers.x) as usize] = result;
                (carry, result)
            }
            _ => panic!("Invalid addressing mode for LSR"),
        };

        // Set the carry flag when losing the 1's place.
        self.set_status(Status::Carry, c);
        self.set_status(Status::Zero, r == 0);
        self.set_status(Status::Negative, false);
    }

    fn nop(&mut self) {}

    fn ora(&mut self) {
        let data = self.fetch();
        self.registers.ac |= data;

        self.set_status(Status::Zero, self.registers.ac == 0);
        self.set_status(Status::Negative, self.registers.ac & 0x80 != 0);
    }

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

    fn rol(&mut self) {
        let data = self.fetch();
        let mode = self.addr_mode;
        let addr = data;

        // If there's a 1 in the 1's place, it will be shifted off.
        let carried = |f: u8| f & 0x01 != 0;

        // TODO: Factor out these matches. Memory or Accumulator OP
        let operation = |d: &mut u8| -> (u8, bool) {
            let r = *d << 1;
            let c = carried(r);
            *d = r;
            (r, c)
        };

        // Perhaps we can use a closure to do this....wtf xD
        // Because flatmap and registers are different types, we can't use a closure simply.
        let (r, c) = match mode {
            Mode::Accumulator => operation(&mut self.registers.ac),
            Mode::ZeroPage => operation(&mut self.flatmap[addr as usize]),
            Mode::ZeroPageX => operation(&mut self.flatmap[(addr + self.registers.x) as usize]),
            Mode::Absolute => operation(&mut self.flatmap[addr as usize]),
            Mode::AbsoluteX => operation(&mut self.flatmap[(addr + self.registers.x) as usize]),
            
            _ => panic!("Invalid addressing mode for ROL"),
        };  

        self.set_status(Status::Carry, c);
        self.set_status(Status::Zero, r == 0);
        self.set_status(Status::Negative, r & 0x80 != 0);
    }

    fn ror(&mut self) {
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
        // TODO: Factor out these matches. Set Heap OP
        match self.addr_mode {
            // Code duplication because we are not able to precalculate whether we're 
            // fetching or setting a heap addr. Because we're adhering to u8 types we 
            // can't simply return the whole address.
            //
            // So, we need to potentially set the heap or fetch from it. If we could 
            // return references, we can return those from a generalized match.
            // 
            // If we can do this here, we can also likely do it for ror/rol functions.
            // We're likely missing a logical step that conforms to the 6502 structure.
            // TODO: whew.
            Mode::ZeroPage => {
                let addr = self.fetch() as usize;
                self.flatmap[addr] = self.registers.y;         // Only 0x00 - 0xFF, self.fetch returns u8 so this is guaranteed.
            }
            Mode::ZeroPageX => {
                let addr = (self.fetch() as usize + self.registers.x as usize) & 0xFF;
                self.flatmap[addr] = self.registers.y; 
            }
            Mode::Absolute => {
                let addr = self.fetch();
                self.flatmap[addr as usize] = self.registers.y;
            }
            _ => panic!("Invalid addressing mode for STY"),
        }
    }

    fn tax(&mut self) {
        self.registers.x = self.registers.ac;
        // TODO: We can definitely create a helper for the transfer instructions.
        self.set_status(Status::Zero, self.registers.x == 0);
        self.set_status(Status::Negative, self.registers.x & 0x80 != 0);
    }

    fn tay(&mut self) {
        self.registers.y = self.registers.ac;
        self.set_status(Status::Zero, self.registers.y == 0);
        self.set_status(Status::Negative, self.registers.y & 0x80 != 0);
    }

    fn tsx(&mut self) {
        self.registers.x = self.registers.sp;
        self.set_status(Status::Zero, self.registers.x == 0);
        self.set_status(Status::Negative, self.registers.x & 0x80 != 0);
    }

    fn txa(&mut self) {
        self.registers.ac = self.registers.x;
        self.set_status(Status::Zero, self.registers.ac == 0);
        self.set_status(Status::Negative, self.registers.ac & 0x80 != 0);
    }

    fn txs(&mut self) {
        self.registers.sp = self.registers.x;
    }

    fn tya(&mut self) {
        self.registers.ac = self.registers.y;
        self.set_status(Status::Zero, self.registers.ac == 0);
        self.set_status(Status::Negative, self.registers.ac & 0x80 != 0);
    }
}
