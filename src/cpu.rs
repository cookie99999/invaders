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
    {0x30, 1, 4, "*NOP"}, {0x31, 3, 10, "LXI"}, {0x32, 3, 13, "STA"}, {0x33, 1, 5, "INX"},
    {0x34, 1, 10, "INR"}, {0x35, 1, 10, "DCR"}, {0x36, 2, 10, "MVI"}, {0x37, 1, 4, "STC"},
    {0x38, 1, 4, "*NOP"}, {0x39, 1, 10, "DAD"}, {0x3a, 3, 13, "LDA"}, {0x3b, 1, 5, "DCX"},
    {0x3c, 1, 5, "INR"}, {0x3d, 1, 5, "DCR"}, {0x3e, 2, 7, "MVI"}, {0x3f, 1, 4, "CMC"},
    {0x40, 1, 5, "MOV"}, {0x41, 1, 5, "MOV"}, {0x42, 1, 5, "MOV"}, {0x43, 1, 5, "MOV"},
    {0x44, 1, 5, "MOV"}, {0x45, 1, 5, "MOV"}, {0x46, 1, 7, "MOV"}, {0x47, 1, 5, "MOV"},
    {0x48, 1, 5, "MOV"}, {0x49, 1, 5, "MOV"}, {0x4a, 1, 5, "MOV"}, {0x4b, 1, 5, "MOV"},
    {0x4c, 1, 5, "MOV"}, {0x4d, 1, 5, "MOV"}, {0x4e, 1, 7, "MOV"}, {0x4f, 1, 5, "MOV"},
    {0x50, 1, 5, "MOV"}, {0x51, 1, 5, "MOV"}, {0x52, 1, 5, "MOV"}, {0x53, 1, 5, "MOV"},
    {0x54, 1, 5, "MOV"}, {0x55, 1, 5, "MOV"}, {0x56, 1, 7, "MOV"}, {0x57, 1, 5, "MOV"},
    {0x58, 1, 5, "MOV"}, {0x59, 1, 5, "MOV"}, {0x5a, 1, 5, "MOV"}, {0x5b, 1, 5, "MOV"},
    {0x5c, 1, 5, "MOV"}, {0x5d, 1, 5, "MOV"}, {0x5e, 1, 7, "MOV"}, {0x5f, 1, 5, "MOV"},
    {0x60, 1, 5, "MOV"}, {0x61, 1, 5, "MOV"}, {0x62, 1, 5, "MOV"}, {0x63, 1, 5, "MOV"},
    {0x64, 1, 5, "MOV"}, {0x65, 1, 5, "MOV"}, {0x66, 1, 7, "MOV"}, {0x67, 1, 5, "MOV"},
    {0x68, 1, 5, "MOV"}, {0x69, 1, 5, "MOV"}, {0x6a, 1, 5, "MOV"}, {0x6b, 1, 5, "MOV"},
    {0x6c, 1, 5, "MOV"}, {0x6d, 1, 5, "MOV"}, {0x6e, 1, 7, "MOV"}, {0x6f, 1, 5, "MOV"},
    {0x70, 1, 7, "MOV"}, {0x71, 1, 7, "MOV"}, {0x72, 1, 7, "MOV"}, {0x73, 1, 7, "MOV"},
    {0x74, 1, 7, "MOV"}, {0x75, 1, 7, "MOV"}, {0x76, 1, 7, "HLT"}, {0x77, 1, 7, "MOV"},
    {0x78, 1, 5, "MOV"}, {0x79, 1, 5, "MOV"}, {0x7a, 1, 5, "MOV"}, {0x7b, 1, 5, "MOV"},
    {0x7c, 1, 5, "MOV"}, {0x7d, 1, 5, "MOV"}, {0x7e, 1, 7, "MOV"}, {0x7f, 1, 5, "MOV"},
    {0x80, 1, 4, "ADD"}, {0x81, 1, 4, "ADD"}, {0x82, 1, 4, "ADD"}, {0x83, 1, 4, "ADD"},
    {0x84, 1, 4, "ADD"}, {0x85, 1, 4, "ADD"}, {0x86, 1, 7, "ADD"}, {0x87, 1, 4, "ADD"},
    {0x88, 1, 4, "ADC"}, {0x89, 1, 4, "ADC"}, {0x8a, 1, 4, "ADC"}, {0x8b, 1, 4, "ADC"},
    {0x8c, 1, 4, "ADC"}, {0x8d, 1, 4, "ADC"}, {0x8e, 1, 7, "ADC"}, {0x8f, 1, 4, "ADC"},
    {0x90, 1, 4, "SUB"}, {0x91, 1, 4, "SUB"}, {0x92, 1, 4, "SUB"}, {0x93, 1, 4, "SUB"},
    {0x94, 1, 4, "SUB"}, {0x95, 1, 4, "SUB"}, {0x96, 1, 7, "SUB"}, {0x97, 1, 4, "SUB"},
    {0x98, 1, 4, "SBB"}, {0x99, 1, 4, "SBB"}, {0x9a, 1, 4, "SBB"}, {0x9b, 1, 4, "SBB"},
    {0x9c, 1, 4, "SBB"}, {0x9d, 1, 4, "SBB"}, {0x9e, 1, 7, "SBB"}, {0x9f, 1, 4, "SBB"},
    {0xa0, 1, 4, "ANA"}, {0xa1, 1, 4, "ANA"}, {0xa2, 1, 4, "ANA"}, {0xa3, 1, 4, "ANA"},
    {0xa4, 1, 4, "ANA"}, {0xa5, 1, 4, "ANA"}, {0xa6, 1, 7, "ANA"}, {0xa7, 1, 4, "ANA"},
    {0xa8, 1, 4, "XRA"}, {0xa9, 1, 4, "XRA"}, {0xaa, 1, 4, "XRA"}, {0xab, 1, 4, "XRA"},
    {0xac, 1, 4, "XRA"}, {0xad, 1, 4, "XRA"}, {0xae, 1, 7, "XRA"}, {0xaf, 1, 4, "XRA"},
    {0xb0, 1, 4, "ORA"}, {0xb1, 1, 4, "ORA"}, {0xb2, 1, 4, "ORA"}, {0xb3, 1, 4, "ORA"},
    {0xb4, 1, 4, "ORA"}, {0xb5, 1, 4, "ORA"}, {0xb6, 1, 7, "ORA"}, {0xb7, 1, 4, "ORA"},
    {0xb8, 1, 4, "CMP"}, {0xb9, 1, 4, "CMP"}, {0xba, 1, 4, "CMP"}, {0xbb, 1, 4, "CMP"},
    {0xbc, 1, 4, "CMP"}, {0xbd, 1, 4, "CMP"}, {0xbe, 1, 7, "CMP"}, {0xbf, 1, 4, "CMP"},
    {0xc0, 1, 5, "RNZ"}, {0xc1, 1, 10, "POP"}, {0xc2, 3, 10, "JNZ"}, {0xc3, 3, 10, "JMP"},
    {0xc4, 3, 11, "CNZ"}, {0xc5, 1, 11, "PUSH"}, {0xc6, 2, 7, "ADI"}, {0xc7, 1, 11, "RST"},
    {0xc8, 1, 5, "RZ"}, {0xc9, 1, 10, "RET"}, {0xca, 3, 10, "JZ"}, {0xcb, 3, 10, "*JMP"},
    {0xcc, 3, 11, "CZ"}, {0xcd, 3, 17, "CALL"}, {0xce, 2, 7, "ACI"}, {0xcf, 1, 11, "RST"},
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

    fn push_word(&mut self, data: u16) {
	let hi = ((data & 0xff00) >> 8) as u8;
	let lo = (data & 0x00ff) as u8;
	self.bus.write_byte(self.sp.wrapping_sub(1), hi);
	self.bus.write_byte(self.sp.wrapping_sub(2), lo);
	self.sp -= 2;
    }

    fn pop_word(&mut self) -> u16 {
	let lo = self.bus.read_byte(self.sp);
	let hi = self.bus.read_byte(self.sp.wrapping_add(1));
	self.sp += 2;
	((hi as u16) << 8) | (lo as u16)
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
	    0b110 => todo!("d = mem"),
	    _ => &mut self.a,
	};

	let cond: bool = match c {
	    0 => !self.f.contains(PSW::Z),
	    1 => self.f.contains(PSW::Z),
	    2 => !self.f.contains(PSW::C),
	    3 => self.f.contains(PSW::C),
	    4 => !self.f.contains(PSW::P),
	    5 => self.f.contains(PSW::P),
	    6 => !self.f.contains(PSW::S),
	    _ => self.f.contains(PSW::S),
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
	    "ADD" => {
		let mut tmp = self.a as u16;
		tmp = tmp.wrapping_add(s as u16);
		self.f.set(PSW::A, ((self.a & 0xf) + (s & 0xf)) > 0x0f);
		self.f.set(PSW::C, tmp > 0xff);
		self.f.set(PSW::Z, (tmp & 0xff) == 0);
		self.f.set(PSW::S, (tmp & 0x80) != 0);
		self.f.set(PSW::P, (((tmp & 0xff) as u8).count_ones() % 2) == 0);
		self.a = tmp as u8;
	    },
	    "ADI" => {
		let mut tmp = self.a as u16;
		tmp = tmp.wrapping_add(op1 as u16);
		self.f.set(PSW::A, ((self.a & 0xf) + (op1 & 0xf)) > 0x0f);
		self.f.set(PSW::C, tmp > 0xff);
		self.f.set(PSW::Z, (tmp & 0xff) == 0);
		self.f.set(PSW::S, (tmp & 0x80) != 0);
		self.f.set(PSW::P, (((tmp & 0xff) as u8).count_ones() % 2) == 0);
		self.a = tmp as u8;
	    },
	    "ADC" => {
		let mut tmp = self.a as u16;
		tmp = tmp.wrapping_add(s as u16).wrapping_add(self.f.contains(PSW::C) as u16);
		self.f.set(PSW::A, ((self.a & 0xf) + (s & 0xf) + self.f.contains(PSW::C) as u8) > 0x0f);
		self.f.set(PSW::C, tmp > 0xff);
		self.f.set(PSW::Z, (tmp & 0xff) == 0);
		self.f.set(PSW::S, (tmp & 0x80) != 0);
		self.f.set(PSW::P, (((tmp & 0xff) as u8).count_ones() % 2) == 0);
		self.a = tmp as u8;
	    },
	    "ACI" => {
		let mut tmp = self.a as u16;
		tmp = tmp.wrapping_add(op1 as u16).wrapping_add(self.f.contains(PSW::C) as u16);
		self.f.set(PSW::A, ((self.a & 0xf) + (op1 & 0xf) + self.f.contains(PSW::C) as u8) > 0x0f);
		self.f.set(PSW::C, tmp > 0xff);
		self.f.set(PSW::Z, (tmp & 0xff) == 0);
		self.f.set(PSW::S, (tmp & 0x80) != 0);
		self.f.set(PSW::P, (((tmp & 0xff) as u8).count_ones() % 2) == 0);
		self.a = tmp as u8;
	    },
	    "SUB" => {
		let mut tmp = self.a as u16;
		tmp = tmp.wrapping_sub(s as u16);
		self.f.set(PSW::A, ((self.a & 0xf) + ((!s & 0xff) & 0xf) + 1) > 0x0f);
		self.f.set(PSW::C, tmp > 0xff);
		self.f.set(PSW::Z, (tmp & 0xff) == 0);
		self.f.set(PSW::S, (tmp & 0x80) != 0);
		self.f.set(PSW::P, (((tmp & 0xff) as u8).count_ones() % 2) == 0);
		self.a = tmp as u8;
	    },
	    "SUI" => {
		let mut tmp = self.a as u16;
		tmp = tmp.wrapping_sub(op1 as u16);
		self.f.set(PSW::A, ((self.a & 0xf) + ((!op1 & 0xff) & 0xf) + 1) > 0x0f);
		self.f.set(PSW::C, tmp > 0xff);
		self.f.set(PSW::Z, (tmp & 0xff) == 0);
		self.f.set(PSW::S, (tmp & 0x80) != 0);
		self.f.set(PSW::P, (((tmp & 0xff) as u8).count_ones() % 2) == 0);
		self.a = tmp as u8;
	    },
	    "SBB" => {
		let mut tmp = self.a as u16;
		tmp = tmp.wrapping_sub(s as u16).wrapping_sub(self.f.contains(PSW::C) as u16);
		self.f.set(PSW::A, ((self.a & 0xf) + ((!s & 0xff) & 0xf) + !self.f.contains(PSW::C) as u8) > 0x0f);
		self.f.set(PSW::C, tmp > 0xff);
		self.f.set(PSW::Z, (tmp & 0xff) == 0);
		self.f.set(PSW::S, (tmp & 0x80) != 0);
		self.f.set(PSW::P, (((tmp & 0xff) as u8).count_ones() % 2) == 0);
		self.a = tmp as u8;
	    },
	    "SBI" => {
		let mut tmp = self.a as u16;
		tmp = tmp.wrapping_sub(op1 as u16).wrapping_sub(self.f.contains(PSW::C) as u16);
		self.f.set(PSW::A, ((self.a & 0xf) + ((!op1 & 0xff) & 0xf) + !self.f.contains(PSW::C) as u8) > 0x0f);
		self.f.set(PSW::C, tmp > 0xff);
		self.f.set(PSW::Z, (tmp & 0xff) == 0);
		self.f.set(PSW::S, (tmp & 0x80) != 0);
		self.f.set(PSW::P, (((tmp & 0xff) as u8).count_ones() % 2) == 0);
		self.a = tmp as u8;
	    },
	    "INR" => {
		let tmp = d.wrapping_add(1) as u16;
		self.f.set(PSW::Z, tmp == 0);
		self.f.set(PSW::S, (tmp & 0x80) != 0);
		self.f.set(PSW::P, (((tmp & 0xff) as u8).count_ones() % 2) == 0);
		self.f.set(PSW::A, ((*d & 0x0f).wrapping_add(1)) > 0x0f);
		*d = tmp as u8;
	    },
	    "DCR" => {
		let tmp = d.wrapping_sub(1) as u16;
		self.f.set(PSW::Z, tmp == 0);
		self.f.set(PSW::S, (tmp & 0x80) != 0);
		self.f.set(PSW::P, (((tmp & 0xff) as u8).count_ones() % 2) == 0);
		self.f.set(PSW::A, (*d & 0x0f) != 0);
		*d = tmp as u8;
	    },
	    "INX" => {
		let tmp = self.read_rp(rp);
		self.write_rp(rp, tmp.wrapping_add(1));
	    },
	    "DCX" => {
		let tmp = self.read_rp(rp);
		self.write_rp(rp, tmp.wrapping_sub(1));
	    },
	    "DAD" => {
		let hltmp = self.read_rp(2);
		let rptmp = self.read_rp(rp);
		let tmp = hltmp.wrapping_add(rptmp) as u32;
		self.f.set(PSW::C, tmp > 0xffff);
		self.write_rp(2, tmp as u16);
	    },
	    "DAA" => {
		let mut tmp = self.a as u16;
		if ((tmp & 0x0f) > 0x09) || self.f.contains(PSW::A) {
		    self.f.set(PSW::A, (((tmp & 0x0f) + 0x06) & 0xf0) != 0);
		    tmp += 6;
		    self.f.set(PSW::C, (tmp & 0xff00) != 0);
		}
		if ((tmp & 0xf0) > 0x90) || self.f.contains(PSW::C) {
		    tmp += 0x60;
		    self.f.set(PSW::C, (tmp & 0xff00) != 0);
		}
		self.f.set(PSW::Z, (tmp & 0xff) == 0);
		self.f.set(PSW::S, (tmp & 0x80) != 0);
		self.f.set(PSW::P, (((tmp & 0xff) as u8).count_ones() % 2) == 0);
		self.a = tmp as u8;
	    },
	    "ANA" => {
		let tmp = self.a & s;
		self.f.set(PSW::C, false);
		self.f.set(PSW::A, ((self.a | s) & 0x08) != 0);
		self.f.set(PSW::Z, tmp == 0);
		self.f.set(PSW::S, (tmp & 0x80) != 0);
		self.f.set(PSW::P, ((tmp & 0xff).count_ones() % 2) == 0);
		self.a = tmp;
	    },
	    "ANI" => {
		let tmp = self.a & op1;
		self.f.set(PSW::C, false);
		self.f.set(PSW::A, false);
		self.f.set(PSW::Z, tmp == 0);
		self.f.set(PSW::S, (tmp & 0x80) != 0);
		self.f.set(PSW::P, ((tmp & 0xff).count_ones() % 2) == 0);
		self.a = tmp;
	    },
	    "XRA" => {
		let tmp = self.a ^ s;
		self.f.set(PSW::C, false);
		self.f.set(PSW::A, false);
		self.f.set(PSW::Z, tmp == 0);
		self.f.set(PSW::S, (tmp & 0x80) != 0);
		self.f.set(PSW::P, ((tmp & 0xff).count_ones() % 2) == 0);
		self.a = tmp;
	    },
	    "XRI" => {
		let tmp = self.a ^ op1;
		self.f.set(PSW::C, false);
		self.f.set(PSW::A, false);
		self.f.set(PSW::Z, tmp == 0);
		self.f.set(PSW::S, (tmp & 0x80) != 0);
		self.f.set(PSW::P, ((tmp & 0xff).count_ones() % 2) == 0);
		self.a = tmp;
	    },
	    "ORA" => {
		let tmp = self.a | s;
		self.f.set(PSW::C, false);
		self.f.set(PSW::A, false);
		self.f.set(PSW::Z, tmp == 0);
		self.f.set(PSW::S, (tmp & 0x80) != 0);
		self.f.set(PSW::P, ((tmp & 0xff).count_ones() % 2) == 0);
		self.a = tmp;
	    },
	    "ORI" => {
		let tmp = self.a | op1;
		self.f.set(PSW::C, false);
		self.f.set(PSW::A, false);
		self.f.set(PSW::Z, tmp == 0);
		self.f.set(PSW::S, (tmp & 0x80) != 0);
		self.f.set(PSW::P, ((tmp & 0xff).count_ones() % 2) == 0);
		self.a = tmp;
	    },
	    "CMP" => {
		let mut tmp = self.a as u16;
		tmp = tmp.wrapping_sub(s as u16);
		self.f.set(PSW::A, ((self.a & 0xf) + ((!s & 0xff) & 0xf) + 1) > 0x0f);
		self.f.set(PSW::C, tmp > 0xff);
		self.f.set(PSW::Z, (tmp & 0xff) == 0);
		self.f.set(PSW::S, (tmp & 0x80) != 0);
		self.f.set(PSW::P, (((tmp & 0xff) as u8).count_ones() % 2) == 0);
	    },
	    "CPI" => {
		let mut tmp = self.a as u16;
		tmp = tmp.wrapping_sub(op1 as u16);
		self.f.set(PSW::A, ((self.a & 0xf) + ((!op1 & 0xff) & 0xf) + 1) > 0x0f);
		self.f.set(PSW::C, tmp > 0xff);
		self.f.set(PSW::Z, (tmp & 0xff) == 0);
		self.f.set(PSW::S, (tmp & 0x80) != 0);
		self.f.set(PSW::P, (((tmp & 0xff) as u8).count_ones() % 2) == 0);
	    },
	    "RLC" => {
		self.f.set(PSW::C, ((self.a & 0x80) >> 7) != 0);
		self.a = self.a << 1;
		self.a = self.a | (self.f.contains(PSW::C) as u8);
	    },
	    "RRC" => {
		self.f.set(PSW::C, (self.a & 1) != 0);
		self.a = ((self.a & 1) << 7) | (self.a >> 1);
	    },
	    "RAL" => {
		let tmp = self.f.contains(PSW::C) as u8;
		self.f.set(PSW::C, ((self.a & 0x80) >> 7) != 0);
		self.a = self.a << 1;
		self.a = self.a | tmp;
	    },
	    "RAR" => {
		let tmp = (self.f.contains(PSW::C) as u8) << 7;
		self.f.set(PSW::C, (self.a & 1) != 0);
		self.a = self.a >> 1;
		self.a = self.a | tmp;
	    },
	    "CMA" => {
		self.a = !self.a;
	    },
	    "CMC" => {
		self.f.toggle(PSW::C);
	    },
	    "STC" => {
		self.f.insert(PSW::C);
	    },
	    "JMP" => {
		self.pc = opw;
	    },
	    "JNZ" | "JZ" | "JNC" | "JC" |
	    "JPO" | "JPE" | "JP" | "JM" => {
		if cond {
		    self.pc = opw;
		}
	    },
	    "CALL" => {
		self.push_word(self.pc);
		self.pc = opw;
	    },
	    "CNZ" | "CZ" | "CNC" | "CC" |
	    "CPO" | "CPE" | "CP" | "CM" => {
		if cond {
		    self.push_word(self.pc);
		    self.pc = opw;
		    self.cycles += 6;
		}
	    },
	    "RET" => {
		self.pc = self.pop_word();
	    },
	    "RNZ" | "RZ" | "RNC" | "RC" |
	    "RPO" | "RPE" | "RP" | "RM" => {
		if cond {
		    self.pc = self.pop_word();
		    self.cycles += 6;
		}
	    },
	    "RST" => {
		self.push_word(self.pc);
		self.pc = (n << 3) as u16;
	    },
	    "PCHL" => {
		self.pc = hlptr;
	    },
	    _ =>
		todo!("Unimplemented instruction {}", instr.mnemonic),
	};

	self.cycles - oldcycles
    }
}
