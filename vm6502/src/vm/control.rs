use bitmatch::bitmatch;
use std::fmt::{Debug, Formatter, Result};

use crate::prelude::*;
//use crate::{pc_from_mode, cycles_from_mode};

pub mod prelude {
    pub use crate::vm::control::InstructionController;
    pub use crate::vm::control::Mode;
}

/// Virtual machine addressing mode enum.
///
#[derive(PartialEq)]
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

impl Debug for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Mode::Accumulator => write!(f, "Accumulator"),
            Mode::Implied => write!(f, "Implied"),
            Mode::Immediate => write!(f, "Immediate"),
            Mode::ZeroPage => write!(f, "ZeroPage"),
            Mode::ZeroPageX => write!(f, "ZeroPageX"),
            Mode::ZeroPageY => write!(f, "ZeroPageY"),
            Mode::Relative => write!(f, "Relative"),
            Mode::Absolute => write!(f, "Absolute"),
            Mode::AbsoluteX => write!(f, "AbsoluteX"),
            Mode::AbsoluteY => write!(f, "AbsoluteY"),
            Mode::Indirect => write!(f, "Indirect"),
            Mode::IndirectX => write!(f, "IndirectX"),
            Mode::IndirectY => write!(f, "IndirectY"),
        }
    }
}

pub trait InstructionController {
    fn tick(&mut self) -> u64;
    // TODO: Abstract matches out of tick so that you can get the ops then tick with opcode.
    // fn opcode(&mut self, op: &str);
    // fn opcode(&mut self, op: u8);

    // TODO: Mode could be a macro, or other macros could be integrated. Consider this API decision more closely.
    fn mode(&mut self, op: u8) -> Mode;
    fn fetch(&mut self) -> u8;
}

/// Virtual machine core control functionality.
///
/// This provides three main internal functions, `tick`, `mode`, and `fetch`.
///
/// # Examples
/// ## `tick`
/// ```
/// use vm6502::prelude::*;
/// let mut vm = VirtualMachine::new();
///
/// vm.insert_program(0x00, "69FFFF");
/// vm.registers.pc = 0x00;
///
/// vm.tick();
///
/// assert_eq!(vm.addr_mode, Mode::Immediate);
/// assert_eq!(vm.flatmap[vm.registers.pc as usize + vm.heap_bounds.0], 0xFF);
/// ```
/// ## `mode`
/// ```
/// use vm6502::prelude::*;
///
/// let mut vm = VirtualMachine::new();
/// let mode = vm.mode(0x69);
///
/// assert_eq!(mode, Mode::Immediate);
/// ```
/// ## `fetch`
/// ```
/// use vm6502::prelude::*;
///
/// let mut vm = VirtualMachine::new();
/// let byte = 0xFF;
///
/// // 0x200 is heap start. See `VirtualMachine::heap_bounds`.
/// vm.flatmap[vm.heap_bounds.0] = 0x69;
/// vm.flatmap[vm.heap_bounds.0 + 1] = byte;
///
/// // Set the mode to immediate. (internal access only)
/// vm.addr_mode = Mode::Immediate;
///
/// let fetched = vm.fetch();
/// assert_eq!(fetched, byte);
/// ```
impl InstructionController for VirtualMachine {
    /// Fetch the next byte from memory using the current address mode and program counter.
    fn fetch(&mut self) -> u8 {
        #[cfg(feature = "show_mode")]
        println!("\n\tfetch mode: {:?}", self.addr_mode);

        let fetched = match self.addr_mode {
            // OPC A
            Mode::Accumulator => self.registers.ac,
            // OPC $LLHH
            // operand is address $HHLL
            Mode::Absolute => {
                let ll = self.flatmap[self.registers.pc as usize + 1];
                let hh = self.flatmap[self.registers.pc as usize + 2];
                self.flatmap[(hh as usize) << 8 | ll as usize]
            }
            // OPC $LLHH,X
            Mode::AbsoluteX => {
                let ll = self.flatmap[self.registers.pc as usize + 1];
                let hh = self.flatmap[self.registers.pc as usize + 2];
                self.flatmap[(hh as usize) << 8 | ll as usize + self.registers.x as usize]
            }
            // OPC $LLHH,Y
            Mode::AbsoluteY => {
                let ll = self.flatmap[self.registers.pc as usize + 1];
                let hh = self.flatmap[self.registers.pc as usize + 2];
                self.flatmap[(hh as usize) << 8 | ll as usize + self.registers.y as usize]
            }
            // OPC #$BB
            Mode::Immediate => {
                self.registers.pc += 1;
                self.flatmap[self.heap_bounds.0 + self.registers.pc as usize]
            }
            // OPC
            Mode::Implied => 0,
            // OPC ($LLHH)
            Mode::Indirect => {
                let ll = self.flatmap[self.registers.pc as usize + 1];
                let hh = self.flatmap[self.registers.pc as usize + 2];
                let addr = (hh as usize) << 8 | ll as usize;
                self.flatmap[addr]
            }
            // OPC ($LL, X)
            // operand is zeropage address; effective address is word in (LL + X, LL + X + 1),
            // inc. without carry: C.w($00LL + X)
            Mode::IndirectX => {
                let ll = self.flatmap[(self.registers.pc + 1) as usize];
                let ell = self.flatmap[ll as usize + self.registers.x as usize];
                let ehh = self.flatmap[ll as usize + self.registers.x as usize + 1];
                let addr = (ehh as usize) << 8 | ell as usize;

                self.flatmap[addr]
            }
            // OPC ($LL), Y
            // operand is zeropage address; effective address is word in (LL, LL + 1)
            // incremented by Y with carry: C.w($00LL) + Y
            // TODO: check if this is correct.
            Mode::IndirectY => {
                let ll = self.flatmap[(self.registers.pc + 1) as usize];
                let ell = self.flatmap[ll as usize];
                let ehh = self.flatmap[ll as usize + 1];
                let addr = (ehh as usize) << 8 | ell as usize + self.registers.y as usize;

                self.flatmap[addr]
            }
            // OPC $BB
            Mode::Relative => {
                let bb = self.flatmap[self.registers.pc as usize + 1];
                // TODO: Write a test for this.
                self.flatmap[self.registers.pc.wrapping_add_signed((bb as i8).into()) as usize]
            }
            // OPC $LL
            Mode::ZeroPage => {
                let ll = self.flatmap[self.registers.pc as usize + 1];
                self.flatmap[ll as usize]
            }
            // OPC $LL, X
            Mode::ZeroPageX => {
                let ll = self.flatmap[self.registers.pc as usize + 1];
                self.flatmap[ll as usize + self.registers.x as usize]
            }
            // OPC $LL, Y
            Mode::ZeroPageY => {
                let ll = self.flatmap[self.registers.pc as usize + 1];
                self.flatmap[ll as usize + self.registers.y as usize]
            }
        };

        #[cfg(feature = "show_fetched")]
        println!(
            "\n\tfetched value: {:02X} by mode: {:?}",
            fetched, self.addr_mode
        );

        fetched
    }

    /// Check the opcode and return the addressing mode.
    #[bitmatch]
    fn mode(&mut self, op: u8) -> Mode {
        #[bitmatch]
        match op {
            "aaabbbcc" => match c {
                0x00 => match b {
                    0x00 => match a {
                        0x00 => Mode::Implied,
                        0x01 => Mode::Absolute,
                        0x02 | 0x03 => Mode::Implied,
                        0x05..=0x07 => Mode::Immediate,
                        _ => panic!("Illegal a value {} for cc0.(b=0x00)", a),
                    },
                    0x01 => match a {
                        0x01 => Mode::ZeroPage,
                        0x04..=0x07 => Mode::ZeroPage,
                        _ => panic!("Illegal a value {:02X} for cc0.(b=0x04..0x07)", a),
                    },
                    0x02 => Mode::Implied,
                    0x03 => match a {
                        0x00 => panic!("Illegal opcode 0x00 for cc0."),
                        0x03 => Mode::Indirect,
                        0x01 | 0x02 | 0x04..=0x07 => Mode::Absolute,
                        _ => panic!("Illegal a value {} for cc0.(b=0x01..0x07)", a),
                    },
                    0x04 => Mode::Relative,
                    0x05 => match a {
                        0x04 | 0x05 => Mode::ZeroPageX,
                        _ => panic!("Illegal a value {} for cc0.(b=0x04|0x05)", a),
                    },
                    0x06 => Mode::Implied,
                    0x07 => match a {
                        0x05 => Mode::AbsoluteX,
                        _ => panic!("Illegal a value {} for cc0.", a),
                    },
                    _ => panic!("Invalid cc0 mode: {}", b),
                },
                0x01 => match b {
                    0x00 => Mode::IndirectX,
                    0x01 => Mode::ZeroPage,
                    0x02 => match a {
                        0x04 => panic!("Illegal opcode 0x04 for cc1.(b=0x02)"),
                        _ => Mode::Immediate,
                    },
                    0x03 => Mode::Absolute,
                    0x04 => Mode::IndirectY,
                    0x05 => Mode::ZeroPageX,
                    0x06 => Mode::AbsoluteY,
                    0x07 => Mode::AbsoluteX,
                    _ => panic!("Invalid cc1 mode: {}", b),
                },
                0x02 => match b {
                    0x00 => match a {
                        0x00 => Mode::Implied,
                        0x05 => Mode::Immediate,
                        _ => panic!("Illegal a value {} for cc2(b=0x00)", a),
                    },
                    0x01 => Mode::ZeroPage,
                    0x02 => match a {
                        0x00..=0x03 => Mode::Accumulator,
                        0x04..=0x07 => Mode::Implied,
                        _ => panic!("Illegal a value {} for cc2(b=0x02)", a),
                    },
                    0x03 => Mode::Absolute,
                    0x04 => Mode::ZeroPageX,
                    0x05 => match a {
                        0x00..=0x03 | 0x06 | 0x07 => Mode::ZeroPageX,
                        0x04 | 0x05 => Mode::ZeroPageY,
                        _ => panic!("Illegal a value {} for cc2.(b=0x05)", a),
                    },
                    0x06 => match a {
                        0x04 | 0x05 => Mode::Implied,
                        _ => panic!("Illegal a value {} for cc2.(b=0x06)", a),
                    },
                    0x07 => match a {
                        0x00..=0x03 | 0x06 | 0x07 => Mode::AbsoluteX,
                        0x05 => Mode::AbsoluteY,
                        _ => panic!("Illegal a value {} for cc2.(b=0x07)", a),
                    },
                    _ => panic!("Invalid cc2 mode: {}", b),
                },
                _ => panic!("Invalid mode: {}", c),
            },
        }
    }

    /// Execute the an arbitrary op. It returns the vm's current `cycle` count.
    #[bitmatch]
    fn tick(&mut self) -> u64 {
        // Get current op
        let op = self.flatmap[self.registers.pc as usize + self.heap_bounds.0];
        // Set internal mode.
        let m = self.mode(op);

        #[cfg(feature = "show_ticked_instrs")]
        println!("ticked over OP=0x{:02X}, {:?}", op, m);

        // Update internal state
        self.addr_mode = m;

        #[allow(unused_variables)]
        #[bitmatch]
        match op {
            "00000000" => {
                #[cfg(feature = "show_vm_instr")]
                println!("BRK");

                self.brk()
            }
            "01000000" => {
                #[cfg(feature = "show_vm_instr")]
                println!("RTI");

                self.rti()
            }
            "01100000" => {
                #[cfg(feature = "show_vm_instr")]
                println!("RTS");

                self.rts()
            }
            "aaabbb01" => {
                #[cfg(feature = "show_vm_tick_arms")]
                println!("\taaabbb01 arm, a={:02X}, b={:02X}", a, b);

                match a {
                    0x00 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tORA");
                        self.ora()
                    }
                    0x01 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tAND");
                        self.and()
                    }
                    0x02 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tEOR");
                        self.eor()
                    }
                    0x03 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tADC");
                        self.adc()
                    }
                    0x04 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tSTA");
                        self.sta()
                    }
                    0x05 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tLDA");
                        self.lda()
                    }
                    0x06 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tCMP");
                        self.cmp()
                    }
                    0x07 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tSBC");
                        self.sbc();
                    }
                    _ => self.nop(),
                }
            }
            "aaabbb10" => {
                #[cfg(feature = "show_vm_tick_arms")]
                println!("\taaabbb10 arm, a={:02X}, b={:02X}", a, b);

                match a {
                    0x00 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tASL");
                        self.asl()
                    }
                    0x01 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tROL");
                        self.rol()
                    }
                    0x02 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tLSR");
                        self.lsr()
                    }
                    0x03 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tROR");
                        self.ror()
                    }
                    0x04 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tSTX");
                        self.stx()
                    }
                    0x05 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tLDX");
                        self.ldx()
                    }
                    0x06 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tDEC");
                        self.dec()
                    }
                    0x07 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tINC");
                        self.inc()
                    }
                    _ => self.nop(),
                }
            }
            "aaabbb00" => {
                #[cfg(feature = "show_vm_tick_arms")]
                println!("\taaa___00 arm, a={:02X}, b={:02X}", a, b);

                match a {
                    0x00 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tBIT");
                        self.bit()
                    }
                    0x01 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tJSR");
                        self.jsr()
                    }
                    0x02 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tJMP");
                        self.jmp()
                    }
                    0x03 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tSTY");
                        self.sty()
                    }
                    0x04 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tLDY");
                        self.ldy()
                    }
                    0x05 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tCPY");
                        self.cpy()
                    }
                    0x06 => {
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tCPX");
                        self.cpx()
                    }
                    _ => self.nop(),
                }
            }
            // conditional jumps = aab10000
            "xxx10000" => {
                #[cfg(feature = "show_vm_tick_arms")]
                println!("\txxx10000 arm, x={:02X}", x);

                match x {
                    0x00 => {
                        self.bpl();
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tBPL");
                    }
                    0x01 => {
                        self.bmi();
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tBMI");
                    }
                    0x02 => {
                        self.bvc();
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tBVC");
                    }
                    0x03 => {
                        self.bvs();
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tBVS");
                    }
                    0x04 => {
                        self.bcc();
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tBCC");
                    }
                    0x05 => {
                        self.bcs();
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tBCS");
                    }
                    0x06 => {
                        self.bne();
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tBNE");
                    }
                    0x07 => {
                        self.beq();
                        #[cfg(feature = "show_vm_instr")]
                        println!("\t\tBEQ");
                    }
                    _ => self.nop(),
                }
            }
            "00001000" => {
                self.php();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tPHP");
            }
            "00101000" => {
                self.plp();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tPLP");
            }
            "01001000" => {
                self.pha();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tPHA");
            }
            "01101000" => {
                self.pla();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tPLA");
            }
            "10001000" => {
                self.dey();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tDEY");
            }
            "10101000" => {
                self.tay();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tTAY");
            }
            "01001100" => {
                self.iny();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tINY");
            }
            "11101000" => {
                self.inx();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tINX");
            }
            "00011000" => {
                self.clc();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tCLC");
            }
            "00111000" => {
                self.sec();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tSEC");
            }
            "01011000" => {
                self.cli();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tCLI");
            }
            "01111000" => {
                self.sei();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tSEI");
            }
            "10011000" => {
                self.tya();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tTYA");
            }
            "10111000" => {
                self.clv();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tCLV");
            }
            "11011000" => {
                self.cld();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tCLD");
            }
            "11111000" => {
                self.sed();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tSED");
            }
            "10001010" => {
                self.txa();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tTXA");
            }
            "10011010" => {
                self.txs();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tTXS");
            }
            "10101010" => {
                self.tax();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tTAX");
            }
            "10111010" => {
                self.tsx();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tTSX");
            }
            "11001010" => {
                self.dex();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tDEX");
            }
            _ => {
                self.nop();
                #[cfg(feature = "show_vm_instr")]
                println!("\t\tNOP");
            }
        };

        self.registers.pc += 1;

        // TODO: This should be updated (along with the PC) by the above commands.
        self.cycles
    }
}
