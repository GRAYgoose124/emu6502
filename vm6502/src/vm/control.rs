use bitmatch::bitmatch;
use std::fmt::{Debug, Formatter, Result};

use crate::check_page_cross;
use crate::prelude::*;

pub mod prelude {
    pub use crate::vm::control::InstructionController;
    pub use crate::vm::control::Mode;
}

/// Virtual machine addressing mode enum.
///
#[derive(PartialEq, Copy, Clone)]
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
    fn step(&mut self) -> u64;
    // TODO: Abstract matches out of step so that you can get the ops then step with opcode.
    // fn opcode(&mut self, op: &str);
    // fn opcode(&mut self, op: u8);

    // TODO: Mode could be a macro, or other macros could be integrated. Consider this API decision more closely.
    fn mode(&mut self, op: u8) -> Mode;
    fn fetch(&mut self) -> u8;
    fn fetch_byte(&mut self) -> u8;
    fn apply(&mut self, address: u16, operation: fn(u8) -> u8) -> u8;

    //
    fn relative_jump(&mut self, offset: u8, cond: bool);
}

/// Virtual machine core control functionality.
///
/// This provides three main internal functions, `step`, `mode`, and `fetch`.
///
/// # Examples
/// ## `step`
/// ```
/// use vm6502::prelude::*;
/// let mut vm = VirtualMachine::new();
///
/// vm.insert_program(0x00, "69FFFF");
/// vm.registers.pc = 0x00;
///
/// vm.step();
///
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
/// // TODO: Unignore this later.
/// ```rust,ignore
/// use vm6502::prelude::*;
///
/// let mut vm = VirtualMachine::new();
/// let byte = 0x01;
///
/// // 0x200 is heap start. See `VirtualMachine::heap_bounds`.
/// vm.set_heap(0x0000, 0x69);
/// vm.set_heap(0x0001, byte);
///
/// assert_ne!(vm.flatmap[0x0001], byte, "Byte {} was not set to 0x0201", byte);
/// assert_eq!(byte, vm.flatmap[0x0201], "Byte {} was not set at 0x0201", byte);
///
/// // Should PC be 0x01 or two here?
/// vm.registers.pc = 0x01;
/// vm.addr_mode = Mode::Immediate;
/// let fetched = vm.fetch();
/// assert_eq!(vm.registers.pc, 0x02, "PC should be incremented by 1 after fetch");
///
/// assert_eq!(fetched, byte, "Fetched byte {} does not match expected byte {}", fetched, byte);
/// ```
impl InstructionController for VirtualMachine {
    // This can probably be combined with fetch byte in some way.
    fn apply(&mut self, address: u16, operation: fn(u8) -> u8) -> u8 {
        let doit = |d: &mut u8| -> u8 {
            let r = operation(*d);
            *d = r;
            r
        };

        let result = match self.addr_mode {
            Mode::Accumulator => doit(&mut self.registers.ac),
            Mode::ZeroPage => doit(&mut self.flatmap[address as usize]),
            Mode::ZeroPageX => {
                doit(&mut self.flatmap[address as usize + self.registers.x as usize])
            }
            Mode::Absolute => doit(&mut self.flatmap[address as usize]),
            Mode::AbsoluteX => {
                doit(&mut self.flatmap[address as usize + self.registers.x as usize])
            }
            Mode::AbsoluteY => {
                doit(&mut self.flatmap[address as usize + self.registers.y as usize])
            }
            Mode::Indirect => doit(&mut self.flatmap[address as usize]),
            Mode::IndirectX => {
                doit(&mut self.flatmap[address as usize + self.registers.x as usize])
            }
            Mode::IndirectY => {
                doit(&mut self.flatmap[address as usize + self.registers.y as usize])
            }
            Mode::Immediate => doit(&mut self.flatmap[address as usize]),
            Mode::Relative => doit(&mut self.flatmap[address as usize]),
            Mode::Implied => doit(&mut self.flatmap[address as usize]),
            Mode::ZeroPageY => {
                doit(&mut self.flatmap[address as usize + self.registers.y as usize])
            }
        };

        result
    }

    /// Fetch the next byte from the program counter.
    fn fetch(&mut self) -> u8 {
        match self.addr_mode {
            Mode::Absolute => {
                // Todo can we move increments to get heap? We have to fix Relative to be
                // parallel. I don't think so, because indirect fetching.
                self.registers.pc += 1;
                let ll = self.get_heap(0) as usize;
                self.registers.pc += 1;
                let hh = self.get_heap(0) as usize;

                let offset = (hh << 2) | ll;
                self.get_heap(offset as u16)
            }
            // OPC $LLHH,X
            Mode::AbsoluteX => {
                self.registers.pc += 1;
                let ll = self.get_heap(0) as usize;
                self.registers.pc += 1;
                let hh = self.get_heap(0) as usize;

                let offset = (hh << 2) | ll + self.registers.x as usize;
                self.get_heap(offset as u16)
            }
            // OPC $LLHH,Y
            Mode::AbsoluteY => {
                self.registers.pc += 1;
                let ll = self.get_heap(0) as usize;
                self.registers.pc += 1;
                let hh = self.get_heap(0) as usize;

                let offset = (hh << 2) | ll + self.registers.y as usize;
                self.get_heap(offset as u16)
            }
            _ => self.fetch_byte(),
        }
    }

    fn fetch_byte(&mut self) -> u8 {
        #[cfg(feature = "show_mode")]
        println!("\n\tfetch mode: {:?}", self.addr_mode);

        // TODO: Implement all PC incrementing.
        let fetched: u8 = match self.addr_mode {
            // OPC A
            Mode::Accumulator => self.registers.ac,
            // OPC $LLHH
            // operand is address $HHLL
            // OPC #$BB
            Mode::Immediate => {
                // TODO: Can we factor pc addition into get_heap?
                // For some reason this is off by one, control.rs:93
                // seems to be showing that we're fetching the previous byte.
                self.registers.pc += 1;
                self.get_heap(0)
            }
            // OPC
            Mode::Implied => 0,
            // OPC ($LLHH)
            Mode::Indirect => {
                self.registers.pc += 1;
                let ll = self.get_heap(0) as usize;
                self.registers.pc += 1;
                let hh = self.get_heap(0) as usize;

                let offset = (hh << 2) | ll;
                self.get_heap(offset as u16)
            }
            // OPC ($LL, X)
            // operand is zeropage address; effective address is word in (LL + X, LL + X + 1),
            // inc. without carry: C.w($00LL + X)
            Mode::IndirectX => {
                self.registers.pc += 1;
                let ll = self.get_heap(0);
                let ell = self.get_heap((ll + self.registers.x) as u16) as usize;
                let ehh = self.get_heap((ll + self.registers.x + 1) as u16) as usize;

                let offset = (ehh << 2) | ell;
                self.get_heap(offset as u16)
            }
            // OPC ($LL), Y
            // operand is zeropage address; effective address is word in (LL, LL + 1)
            // incremented by Y with carry: C.w($00LL) + Y
            // TODO: check if this is correct.
            Mode::IndirectY => {
                self.registers.pc += 1;
                let ll = self.get_heap(0);
                let ell = self.get_heap(ll as u16);
                let ehh = self.get_heap(ll as u16);

                let offset = (ehh << 2) | ell + self.registers.y;
                self.get_heap(offset as u16)
            }
            // OPC $BB
            Mode::Relative => {
                // TODO: Check if i should be setting this
                //self.registers.pc += 1;
                self.get_heap(1)
            }
            // OPC $LL
            Mode::ZeroPage => {
                self.registers.pc += 1;
                let ll = self.get_heap(0);
                self.get_heap(ll as u16)
            }
            // OPC $LL, X
            Mode::ZeroPageX => {
                self.registers.pc += 1;
                let ll = self.get_heap(0);
                self.get_heap((ll + self.registers.x).into())
            }
            // OPC $LL, Y
            Mode::ZeroPageY => {
                self.registers.pc += 1;
                let ll = self.get_heap(0);
                self.get_heap((ll + self.registers.y).into())
            }
            _ => panic!(
                "No way to be here in this address mode! {:?}",
                self.addr_mode
            ),
        };

        #[cfg(feature = "show_fetched")]
        println!(
            "\n\tfetched value: {:02X} by mode: {:?}",
            fetched, self.addr_mode
        );

        fetched
    }

    // This is setting the offset for branch instructions inside of step(). (TODO: refactor step into get_op, step, then add run.)
    // Because we set the offset here, we don't set it in fetch(), instead we call it.
    // TODO convert all self.flatmap[self.heap_bounds.0 + ....] to a self::HeapInterface.read() fn call
    fn relative_jump(&mut self, fetched: u8, cond: bool) {
        let offset = fetched as i16;
        let newpc = self.registers.pc.wrapping_add_signed(offset);
        self.cycles += 2;

        #[cfg(feature = "show_relative_offset")]
        println!("\t\tRelative jump: 0x{:02X}", offset);

        if cond {
            self.cycles += if check_page_cross!(self, newpc) { 2 } else { 1 };
            self.registers.pc = newpc;

            #[cfg(feature = "show_relative_offset")]
            println!("\t\t\tRelative jump taken. PC: {:04X}", self.registers.pc);
        }
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
    fn step(&mut self) -> u64 {
        // Get current op TODO: Implement internal virtual bounds.
        let op = self.get_heap(0);
        // Set internal mode.
        let m = self.mode(op);

        #[cfg(feature = "show_ticked_instrs")]
        println!("ticked over OP=0x{:02X}, {:?}", op, m);

        // Update internal state
        self.addr_mode = m;

        // Increment PC for the OP fetched.
        // Logic says this should be done before, but maybe after?
        if self.addr_mode != Mode::Relative {
            self.registers.pc = self.registers.pc.wrapping_add(1);
            if self.registers.pc == 0 {
                self.registers.pc = self.heap_bounds.0 as u16;
            }
        }

        // Push the current program counter to the stack for a relative jump.
        // This is for procedures, move.
        /*if self.addr_mode == Mode::Relative {
            let old_ac = self.registers.ac;
            let bytes = self.registers.pc.to_be_bytes();

            self.registers.ac = bytes[1];
            self.push();
            self.registers.ac = bytes[0];
            self.push();

            self.registers.ac = old_ac;
        }*/

        #[allow(unused_variables)]
        #[bitmatch]
        match op {
            "00000000" => {
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("BRK");
                self.brk()
            }
            "01000000" => {
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("RTI");
                self.rti()
            }
            "01100000" => {
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("RTS");
                self.rts()
            }
            "aaabbb01" => {
                #[cfg(feature = "show_vm_tick_arms")]
                println!("\taaabbb01 arm, a={:02X}, b={:02X}", a, b);

                match a {
                    0x00 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tORA");
                        self.ora()
                    }
                    0x01 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tAND");
                        self.and()
                    }
                    0x02 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tEOR");
                        self.eor()
                    }
                    0x03 => {
                        //#[cfg(feature = "show_vm_instr_tick_match")]
                        //println!("\t\tADC");
                        self.adc()
                    }
                    0x04 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tSTA");
                        self.sta()
                    }
                    0x05 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tLDA");
                        self.lda()
                    }
                    0x06 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tCMP");
                        self.cmp()
                    }
                    0x07 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
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
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tASL");
                        self.asl()
                    }
                    0x01 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tROL");
                        self.rol()
                    }
                    0x02 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tLSR");
                        self.lsr()
                    }
                    0x03 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tROR");
                        self.ror()
                    }
                    0x04 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tSTX");
                        self.stx()
                    }
                    0x05 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tLDX");
                        self.ldx()
                    }
                    0x06 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tDEC");
                        self.dec()
                    }
                    0x07 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tINC");
                        self.inc()
                    }
                    _ => self.nop(),
                }
            }
            "aaabbb00" => {
                #[cfg(feature = "show_vm_tick_arms")]
                println!("\taaabbb00 arm, a={:02X}, b={:02X}", a, b);

                // This is the only arm that triggers when op maps to Mode::Relative.
                // Therefore, lets make the assumption that we can do the relative offset calculation here instead of in the fetch function.
                // We are setting the PC
                let offset = self.fetch();
                println!("\t    Relative offset: 0x{:02X}", offset);

                if b == 0b100 {
                    match a {
                        0x00 => {
                            self.bpl(offset);
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tBPL");
                        }
                        0x01 => {
                            self.bmi(offset);
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tBMI");
                        }
                        0x02 => {
                            self.bvc(offset);
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tBVC");
                        }
                        0x03 => {
                            self.bvs(offset);
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tBVS");
                        }
                        0x04 => {
                            self.bcc(offset);
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tBCC");
                        }
                        0x05 => {
                            self.bcs(offset);
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tBCS");
                        }
                        0x06 => {
                            self.bne(offset);
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tBNE");
                        }
                        0x07 => {
                            self.beq(offset);
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tBEQ");
                        }
                        _ => self.nop(),
                    }
                } else {
                    match a {
                        0x00 => {
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tBIT");
                            self.bit()
                        }
                        0x01 => {
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tJSR");
                            self.jsr()
                        }
                        0x02 => {
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tJMP");
                            self.jmp()
                        }
                        0x03 => {
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tSTY");
                            self.sty()
                        }
                        0x04 => {
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tLDY");
                            self.ldy()
                        }
                        0x05 => {
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tCPY");
                            self.cpy()
                        }
                        0x06 => {
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tCPX");
                            self.cpx()
                        }
                        _ => self.nop(),
                    }
                }
            }
            // conditional jumps = aab10000
            "xxx10000" => {
                #[cfg(feature = "show_vm_tick_arms")]
                println!("\txxx10000 arm, x={:02X}", x);
            }
            "00001000" => {
                self.php();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tPHP");
            }
            "00101000" => {
                self.plp();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tPLP");
            }
            "01001000" => {
                self.pha();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tPHA");
            }
            "01101000" => {
                self.pla();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tPLA");
            }
            "10001000" => {
                self.dey();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tDEY");
            }
            "10101000" => {
                self.tay();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tTAY");
            }
            "01001100" => {
                self.iny();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tINY");
            }
            "11101000" => {
                self.inx();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tINX");
            }
            "00011000" => {
                self.clc();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tCLC");
            }
            "00111000" => {
                self.sec();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tSEC");
            }
            "01011000" => {
                self.cli();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tCLI");
            }
            "01111000" => {
                self.sei();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tSEI");
            }
            "10011000" => {
                self.tya();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tTYA");
            }
            "10111000" => {
                self.clv();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tCLV");
            }
            "11011000" => {
                self.cld();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tCLD");
            }
            "11111000" => {
                self.sed();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tSED");
            }
            "10001010" => {
                self.txa();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tTXA");
            }
            "10011010" => {
                self.txs();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tTXS");
            }
            "10101010" => {
                self.tax();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tTAX");
            }
            "10111010" => {
                self.tsx();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tTSX");
            }
            "11001010" => {
                self.dex();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tDEX");
            }
            _ => {
                self.nop();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tNOP");
            }
        };

        #[cfg(feature = "show_vm_post_op")]
        println!("{:?}", self);

        // This should be counting the consumed ops.
        self.cycles += 1;

        // TODO: This should be updated (along with the PC) by the above commands.
        self.cycles
    }
}
