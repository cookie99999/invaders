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
    {0x10, 1, 4, "*NOP"}, {0x11, 3, 10, "LXI"}, {0x12, 1, 7, "STAX"}, {0x13, 1, 5, "INX"},
    {0x14, 1, 5, "INR"}, {0x15, 1, 5, "DCR"}, {0x16, 2, 7, "MVI"}, {0x17, 1, 4, "RAL"},
    {0x18, 1, 4, "*NOP"}, {0x19, 1, 10, "DAD"}, {0x1a, 1, 7, "LDAX"}, {0x1b, 1, 5, "DCX"},
    {0x1c, 1, 5, "INR"}, {0x1d, 1, 5, "DCR"}, {0x1e, 2, 7, "MVI"}, {0x1f, 1, 4, "RAR"},
    {0x20, 1, 4, "*NOP"}, {0x21, 3, 10, "LXI"}, {0x22, 3, 16, "SHLD"}, {0x23, 1, 5, "INX"},
    {0x24, 1, 5, "INR"}, {0x25, 1, 5, "DCR"}, {0x26, 2, 7, "MVI"}, {0x27, 1, 4, "DAA"},
    {0x28, 1, 4, "*NOP"}, {0x29, 1, 10, "DAD"}, {0x2a, 3, 16, "LHLD"}, {0x2b, 1, 5, "DCX"},
    {0x2c, 1, 5, "INR"}, {0x2d, 1, 5, "DCR"}, {0x2e, 2, 7, "MVI"}, {0x2f, 1, 4, "CMA"},
];

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug)]
    struct PSW: u8 {
	const S = 0b10000000;
	const Z = 0b01000000;
	const F5 = 0b00100000;
	const A = 0b00010000;
	const F3 = 0b00001000;
	const P = 0b00000100;
	const F1 = 0b00000010;
	const C = 0b00000001;
    }
}

impl PSW {
    pub fn as_u8(&self) -> u8 {
	self.bits() as u8
    }
}

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
    f: PSW,
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
	    f: PSW::from_bits(0).unwrap(),
	    ime: false,
	    bus: Box::new(bus::InvBus::new()),
	    instr_set: &INSTR_SET_INTEL,
	    cycles: 0,
	}
    }

    pub fn reset(&mut self) {
	self.pc = 0;
	self.ime = false;
    }

    fn read_rp(&self, rp: u8) -> u16 {
	let rpl = match rp {
	    0 => self.c,
	    1 => self.e,
	    2 => self.l,
	    _ => (self.sp & 0xff) as u8,
	};
	let rph = match rp {
	    0 => self.b,
	    1 => self.d,
	    2 => self.h,
	    _ => ((self.sp >> 8) & 0xff) as u8,
	};
	((rph as u16) << 8) | rpl as u16
    }

    fn write_rp(&mut self, rp: u8, data: u16) {
	let hi = ((data >> 8) & 0xff) as u8;
	let lo = (data & 0xff) as u8;

	match rp {
	    0 => {
		self.b = hi;
		self.c = lo;
	    },
	    1 => {
		self.d = hi;
		self.e = lo;
	    },
	    2 => {
		self.h = hi;
		self.l = lo;
	    },
	    _ => {
		self.sp = data;
	    },
	};
    }

    pub fn step(&mut self) -> usize {
	let oldcycles = self.cycles;
	let opcode: u8 = self.bus.read_byte(self.pc);
	//todo: decode extended opcodes
	let instr: &Instruction = &self.instr_set[opcode as usize];
	self.cycles += instr.cycles as usize;
	let d = (opcode >> 3) & 7;
	let s = opcode & 7;
	let rp = (opcode >> 4) & 3;
	let c = d;
	let n = d;
	let hlptr = self.read_rp(2);

	let s = match s {
	    0b000 => self.b,
	    0b001 => self.c,
	    0b010 => self.d,
	    0b011 => self.e,
	    0b100 => self.h,
	    0b101 => self.l,
	    0b110 => self.bus.read_byte(hlptr),
	    _ => self.a,
	};

	let d: &mut u8 = match d {
	    0b000 => &mut self.b,
	    0b001 => &mut self.c,
	    0b010 => &mut self.d,
	    0b011 => &mut self.e,
	    0b100 => &mut self.h,
	    0b101 => &mut self.l,
	    0b110 => todo!("s = mem"),
	    _ => &mut self.a,
	};

	let op1 = self.bus.read_byte(self.pc + 1);
	let op2 = self.bus.read_byte(self.pc + 2);
	let opw = ((op2 as u16) << 8) | op1 as u16;

	self.pc += instr.bytes as u16;
	
	match instr.mnemonic {
	    "MOV" => {
		*d = s;
	    },
	    "MVI" => {
		*d = op1;
	    },
	    "LXI" => {
		self.write_rp(rp, opw);
	    },
	    "LDA" => {
		self.a = self.bus.read_byte(opw);
	    },
	    "STA" => {
		self.bus.write_byte(opw, self.a);
	    },
	    "LHLD" => {
		let tmp = self.bus.read_word(opw);
		self.write_rp(2, tmp);
	    },
	    "SHLD" => {
		let tmp = self.read_rp(2);
		self.bus.write_word(opw, tmp);
	    },
	    "LDAX" => { //todo: only bc and de should be allowed
		let tmp = self.read_rp(rp);
		self.a = self.bus.read_byte(tmp);
	    },
	    "STAX" => { // ""
		let tmp = self.read_rp(rp);
		self.bus.write_byte(tmp, self.a);
	    },
	    "XCHG" => {
		let tmp = self.h;
		self.h = self.d;
		self.d = tmp;
		let tmp = self.l;
		self.l = self.e;
		self.e = tmp;
	    },
	    _ =>
		todo!("Unimplemented instruction {}", instr.mnemonic),
	};

	self.cycles - oldcycles
    }
}
