extern crate bitflags;

use crate::bus;
use crate::bus::Bus;

#[derive(Debug)]
struct Instruction {
    opcode: u8,
    //opcode2: u8,
    bytes: u8,
    cycles: u8,
    mnemonic: &'static str,
}

macro_rules! instr_set {
    ($({ $o: expr, $b: expr, $c: expr, $mn: expr }),* $(,)?) => {
	[
	    $(Instruction { opcode: $o, bytes: $b, cycles: $c, mnemonic: $mn }),*
	]
    };
}

const INSTR_SET_INTEL: [Instruction; 256] = instr_set![
    {0x00, 1, 4, "NOP"}, {0x01, 3, 10, "LXI"}, {0x02, 1, 7, "STAX"}, {0x03, 1, 5, "INX"},
    {0x04, 1, 5, "INR"}, {0x05, 1, 5, "DCR"}, {0x06, 2, 7, "MVI"}, {0x07, 1, 4, "RLC"},
    {0x08, 1, 4, "*NOP"}, {0x09, 1, 10, "DAD"}, {0x0a, 1, 7, "LDAX"}, {0x0b, 1, 5, "DCX"},
    {0x0c, 1, 5, "INR"}, {0x0d, 1, 5, "DCR"}, {0x0e, 2, 7, "MVI"}, {0x0f, 1, 4, "RRC"},
];

pub struct Cpu {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    f: u8,
    ime: bool,
    bus: Box<dyn bus::Bus>,
    instr_set: &'static [Instruction; 256],
    cycles: usize,
}

impl Cpu {
    pub fn new() -> Self {
	Cpu {
	    a: 0,
	    b: 0,
	    c: 0,
	    d: 0,
	    e: 0,
	    h: 0,
	    l: 0,
	    sp: 0,
	    pc: 0,
	    f: 0,
	    ime: false,
	    bus: bus::InvBus::new(),
	    instr_set: &INSTR_SET_INTEL,
	    cycles: 0,
	}
    }

    pub fn step(&mut self) -> usize {
	let oldcycles = self.cycles;
	let opcode: u8 = self.bus.read_byte(self.pc);
	//todo: decode extended opcodes
	let instr: &Instruction = &self.instr_set[opcode as usize];
	self.cycles += instr.cycles as usize;

	self.cycles - oldcycles
    }
}
