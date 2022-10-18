use bitmatch::bitmatch;

use crate::prelude::*;

pub mod prelude {
   pub use crate::instructions::{Instructions, Mode};
}

pub enum Mode {
    Accumulator,
    Implied,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}


pub trait Instructions {
    fn match_instr(&mut self, instr: usize);
    fn set_cc0_mode(&mut self, op: u8, mode: u8);
    fn set_cc1_mode(&mut self, op: u8, mode: u8);
    fn set_cc2_mode(&mut self, op: u8, mode: u8);

    fn adc(&mut self);
    fn and(&mut self);
    fn asl(&mut self);
    fn bcc(&mut self);
    fn bcs(&mut self);
    fn beq(&mut self);
    fn bit(&mut self);
    fn bmi(&mut self);
    fn bne(&mut self);
    fn bpl(&mut self);
    fn brk(&mut self);
    fn bvc(&mut self);
    fn bvs(&mut self);
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

impl Instructions for VirtM {
    fn adc(&mut self) {
        let value = self.fetch(0); // Fetch is directed by the internal mode.

        let result = self.registers.ac as u16 + value as u16;

        let (carry, carried) = (result > 0xFF, result & 0xFF);
        self.registers.ac = carried as u8;

        self.set_status(Status::Carry, carry);
    }

    fn and(&mut self) {
        todo!();
    }

    fn asl(&mut self) {
        todo!();
    }

    fn bcc(&mut self) {
        todo!();
    }

    fn bcs(&mut self) {
        todo!();
    }

    fn beq(&mut self) {
        todo!();
    }

    fn bit(&mut self) {
        todo!();
    }

    fn bmi(&mut self) {
        todo!();
    }

    fn bne(&mut self) {
        todo!();
    }

    fn bpl(&mut self) {
        todo!();
    }

    fn brk(&mut self) {
        todo!();
    }

    fn bvc(&mut self) {
        todo!();
    }

    fn bvs(&mut self) {
        todo!();
    }

    fn clc(&mut self) {
        todo!();
    }

    fn cld(&mut self) {
        todo!();
    }

    fn cli(&mut self) {
        todo!();
    }

    fn clv(&mut self) {
        todo!();
    }

    fn cmp(&mut self) {
        todo!();
    }

    fn cpx(&mut self) {
        todo!();
    }

    fn cpy(&mut self) {
        todo!();
    }

    fn dec(&mut self) {
        todo!();
    }

    fn dex(&mut self) {
        todo!();
    }

    fn dey(&mut self) {
        todo!();
    }

    fn eor(&mut self) {
        todo!();
    }

    fn inc(&mut self) {
        todo!();
    }

    fn inx(&mut self) {
        todo!();
    }

    fn iny(&mut self) {
        todo!();
    }

    fn jmp(&mut self) {
        todo!();
    }

    fn jsr(&mut self) {
        todo!();
    }

    fn lda(&mut self) {
        todo!();
    }

    fn ldx(&mut self) {
        todo!();
    }

    fn ldy(&mut self) {
        todo!();
    }

    fn lsr(&mut self) {
        todo!();
    }

    fn nop(&mut self) {
    }

    fn ora(&mut self) {
        todo!();
    }

    fn pha(&mut self) {
        todo!();
    }

    fn php(&mut self) {
        todo!();
    }

    fn pla(&mut self) {
        todo!();
    }

    fn plp(&mut self) {
        todo!();
    }

    fn rol(&mut self) {
        todo!();
    }

    fn ror(&mut self) {
        todo!();
    }

    fn rti(&mut self) {
        todo!();
    }

    fn rts(&mut self) {
        todo!();
    }

    fn sbc(&mut self) {
        todo!();
    }

    fn sec(&mut self) {
        todo!();
    }

    fn sed(&mut self) {
        todo!();
    }

    fn sei(&mut self) {
        todo!();
    }

    fn sta(&mut self) {
        todo!();
    }

    fn stx(&mut self) {
        todo!();
    }

    fn sty(&mut self) {
        todo!();
    }

    fn tax(&mut self) {
        todo!();
    }

    fn tay(&mut self) {
        todo!();
    }

    fn tsx(&mut self) {
        todo!();
    }

    fn txa(&mut self) {
        todo!();
    }

    fn txs(&mut self) {
        todo!();
    }

    fn tya(&mut self) {
        todo!();
    }

    fn set_cc1_mode(&mut self, op: u8, mode: u8) {
        self.addr_mode = match mode {
            0x00 => Mode::IndirectX,
            0x01 => Mode::ZeroPage,
            0x02 => {
                match op {
                    0x04 => panic!("Illegal opcode 0x04 for Immediate mode."),
                    _ => Mode::Immediate
                }
            }
            0x03 => Mode::Absolute,
            0x04 => Mode::IndirectY,
            0x05 => Mode::ZeroPageX,
            0x06 => Mode::AbsoluteY,
            0x07 => Mode::AbsoluteX,
            _ => panic!("Invalid cc1 mode: {}", mode),
        }
    }

    fn set_cc2_mode(&mut self, op: u8, mode: u8) {
        self.addr_mode = match mode {
            0x00 => {
                match op {
                    0x05 => Mode::Immediate,
                    _ => panic!("Illegal opcode 0x{:02X} for ZeroPage mode.", op),
                }
            }
            0x01 => Mode::ZeroPage,
            0x02 => {
                match op {
                    0x00..=0x03 => Mode::Accumulator,
                    0x04..=0x07 => Mode::Implied,
                    _ => panic!("Illegal opcode 0x{:02X} for Accumulator/cargccccImplied mode.", op),
                }
            },
            0x03 => Mode::Absolute,
            0x04 => Mode::ZeroPageX,
            0x05 => Mode::AbsoluteX,
            _ => panic!("Invalid cc2 mode: {}", mode),
        }
    }

    fn set_cc0_mode(&mut self, _op: u8, mode: u8) {
        self.addr_mode = match mode {     
            0x00 => Mode::IndirectX,       
            _ => panic!("Invalid cc0 mode: {}", mode),
        }
    }

    #[bitmatch]
    fn match_instr(&mut self, addr: usize) {
        #[bitmatch]
        match self.flatmap[addr] {
            "00000000" => self.brk(),
            "00100000" => self.jsr(), // absolute jsr
            "01000000" => self.rti(),
            "01100000" => self.rts(),
            // cc = 01
            "ooommm01" => {
                self.set_cc1_mode(o, m);
                match o {
                    0x00 => self.ora(),   
                    0x01 => self.and(), 
                    0x02 => self.eor(), 
                    0x03 => self.adc(), 
                    0x04 => self.sta(), 
                    0x05 => self.lda(),  
                    0x06 => self.cmp(),
                    0x07 => self.sbc(),
                    _ => self.nop(),
                }
            },
            // cc = 10
            "ooommm10" => {
                self.set_cc2_mode(o, m);
                match o {
                    0x00 => self.asl(),   
                    0x01 => self.rol(), 
                    0x02 => self.lsr(), 
                    0x03 => self.ror(), 
                    0x04 => self.stx(), 
                    0x05 => self.ldx(),  
                    0x06 => self.dec(),
                    0x07 => self.inc(),
                    _ => self.nop()
                }
            },
            // cc = 00
            "ooommm00" => {
                self.set_cc0_mode(o, m);
                match o {
                    0x00 => self.bit(),   
                    0x01 => self.jmp(), 
                    0x02 => self.jmp(), 
                    0x03 => self.sty(), 
                    0x04 => self.ldy(), 
                    0x05 => self.cpy(),  
                    0x06 => self.cpx(),
                    _ => self.nop()
                }
            },
            // conditional jumps = xxy10000
            "00010000" => self.bpl(),
            "00110000" => self.bmi(),
            "01010000" => self.bvc(),
            "01110000" => self.bvs(),
            "10010000" => self.bcc(),
            "10110000" => self.bcs(),
            "11010000" => self.bne(),
            "11110000" => self.beq(),
            // no pattern
            "00001000" => self.php(), 
            "00101000" => self.plp(), 
            "01001000" => self.pha(), 
            "01101000" => self.pla(),
            "10001000" => self.dey(), 
            "10101000" => self.tay(), 
            "01001100" => self.iny(), 
            "11101000" => self.inx(),
            "00011000" => self.clc(),
            "00111000" => self.sec(),
            "01011000" => self.cli(),
            "01111000" => self.sei(),
            "10011000" => self.tya(),
            "10111000" => self.clv(),
            "11011000" => self.cld(),
            "11111000" => self.sed(),
            "10001010" => self.txa(),
            "10011010" => self.txs(),
            "10101010" => self.tax(),
            "10111010" => self.tsx(),
            "11001010" => self.dex(),
            "11101010" => self.nop(),
            _ => self.nop(),
        };
    }  
}