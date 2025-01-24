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
    {0xd0, 1, 5, "RNC"}, {0xd1, 1, 10, "POP"}, {0xd2, 3, 10, "JNC"}, {0xd3, 2, 10, "OUT"},
    {0xd4, 3, 11, "CNC"}, {0xd5, 1, 11, "PUSH"}, {0xd6, 2, 7, "SUI"}, {0xd7, 1, 11, "RST"},
    {0xd8, 1, 5, "RC"}, {0xd9, 1, 10, "*RET"}, {0xda, 3, 10, "JC"}, {0xdb, 2, 10, "IN"},
    {0xdc, 3, 11, "CC"}, {0xdd, 3, 17, "*CALL"}, {0xde, 2, 7, "SBI"}, {0xdf, 1, 11, "RST"},
    {0xe0, 1, 5, "RPO"}, {0xe1, 1, 10, "POP"}, {0xe2, 3, 10, "JPO"}, {0xe3, 1, 18, "XTHL"},
    {0xe4, 3, 11, "CPO"}, {0xe5, 1, 11, "PUSH"}, {0xe6, 2, 7, "ANI"}, {0xe7, 1, 11, "RST"},
    {0xe8, 1, 5, "RPE"}, {0xe9, 1, 5, "PCHL"}, {0xea, 3, 10, "JPE"}, {0xeb, 1, 5, "XCHG"},
    {0xec, 3, 11, "CPE"}, {0xed, 3, 17, "*CALL"}, {0xee, 2, 7, "XRI"}, {0xef, 1, 11, "RST"},
    {0xf0, 1, 5, "RP"}, {0xf1, 1, 10, "POP"}, {0xf2, 3, 10, "JP"}, {0xf3, 1, 4, "DI"},
    {0xf4, 3, 11, "CP"}, {0xf5, 1, 11, "PUSH"}, {0xf6, 2, 7, "ORI"}, {0xf7, 1, 11, "RST"},
    {0xf8, 1, 5, "RM"}, {0xf9, 1, 5, "SPHL"}, {0xfa, 3, 10, "JM"}, {0xfb, 1, 4, "EI"},
    {0xfc, 3, 11, "CM"}, {0xfd, 3, 17, "*CALL"}, {0xfe, 2, 7, "CPI"}, {0xff, 1, 11, "RST"},
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
    pub pc: u16,
    f: PSW,
    ime: bool,
    pub bus: Box<dyn bus::Bus>,
    instr_set: &'static [Instruction; 256],
    cycles: usize,
}

impl Cpu {
    pub fn new(cpm: bool) -> Self {
	if cpm {
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
	    f: PSW::F1,
	    ime: false,
	    bus: Box::new(bus::CpmBus::new()),
	    instr_set: &INSTR_SET_INTEL,
	    cycles: 0,
	    }
	} else {
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
		f: PSW::F1,
		ime: false,
		bus: Box::new(bus::InvBus::new()),
		instr_set: &INSTR_SET_INTEL,
		cycles: 0,
	    }
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
	self.sp = self.sp.wrapping_sub(2);
    }

    fn pop_word(&mut self) -> u16 {
	let lo = self.bus.read_byte(self.sp);
	let hi = self.bus.read_byte(self.sp.wrapping_add(1));
	self.sp = self.sp.wrapping_add(2);
	((hi as u16) << 8) | (lo as u16)
    }

    pub fn step(&mut self) -> usize {
	let oldcycles = self.cycles;
	let opcode: u8 = self.bus.read_byte(self.pc);
	//todo: decode extended opcodes
	let instr: &Instruction = &self.instr_set[opcode as usize];
	self.cycles += instr.cycles as usize;
	let d_bits = (opcode >> 3) & 7;
	let s = opcode & 7;
	let rp = (opcode >> 4) & 3;
	let c = d_bits;
	let n = d_bits;
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

	let d: &mut u8 = match d_bits {
	    0b000 => &mut self.b,
	    0b001 => &mut self.c,
	    0b010 => &mut self.d,
	    0b011 => &mut self.e,
	    0b100 => &mut self.h,
	    0b101 => &mut self.l,
	    //0b110 => todo!("d = mem"),
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

	let op1 = self.bus.read_byte(self.pc.wrapping_add(1));
	let op2 = self.bus.read_byte(self.pc.wrapping_add(2));
	let opw = ((op2 as u16) << 8) | op1 as u16;

	disas(self.pc, instr.opcode, op1, op2, opw);

	self.pc = self.pc.wrapping_add(instr.bytes as u16);
	
	match instr.mnemonic {
	    "MOV" => {
		if d_bits == 6 {
		    self.bus.write_byte(hlptr, s);
		} else {
		    *d = s;
		}
	    },
	    "MVI" => {
		if d_bits == 6 {
		    self.bus.write_byte(hlptr, op1);
		} else {
		    *d = op1;
		}
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
		let mut tmp = d.wrapping_add(1) as u16;
		if d_bits == 6 {
		    tmp = self.bus.read_byte(hlptr).wrapping_add(1) as u16;
		}
		self.f.set(PSW::Z, tmp == 0);
		self.f.set(PSW::S, (tmp & 0x80) != 0);
		self.f.set(PSW::P, (((tmp & 0xff) as u8).count_ones() % 2) == 0);
		self.f.set(PSW::A, ((*d & 0x0f).wrapping_add(1)) > 0x0f);
		if d_bits == 6 {
		    self.bus.write_byte(hlptr, tmp as u8);
		} else {
		    *d = tmp as u8;
		}
	    },
	    "DCR" => {
		let mut tmp = d.wrapping_sub(1) as u16;
		if d_bits == 6 {
		    tmp = self.bus.read_byte(hlptr).wrapping_sub(1) as u16;
		}
		self.f.set(PSW::Z, tmp == 0);
		self.f.set(PSW::S, (tmp & 0x80) != 0);
		self.f.set(PSW::P, (((tmp & 0xff) as u8).count_ones() % 2) == 0);
		self.f.set(PSW::A, (*d & 0x0f) != 0);
		if d_bits == 6 {
		    self.bus.write_byte(hlptr, tmp as u8);
		} else {
		    *d = tmp as u8;
		}
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
	    "JMP" | "*JMP" => {
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
	    "PUSH" => {
		let mut tmp = self.read_rp(rp);
		if rp == 3 {
		    tmp = ((self.a as u16) << 8) | (self.f.as_u8() as u16);
		}
		self.push_word(tmp);
	    },
	    "POP" => {
		let tmp = self.pop_word();
		if rp != 3 {
		    self.write_rp(rp, tmp);
		} else {
		    self.a = ((tmp & 0xff00) >> 8) as u8;
		    self.f = PSW::from_bits((tmp & 0x00ff) as u8).unwrap();
		}
	    },
	    "XTHL" => {
		let tmp = self.pop_word();
		let hl = self.read_rp(2);
		self.push_word(hl);
		self.write_rp(2, tmp);
	    },
	    "SPHL" => {
		self.sp = self.read_rp(2);
	    },
	    "IN" => {
		self.a = self.bus.read_io_byte(op1);
	    },
	    "OUT" => {
		self.bus.write_io_byte(op1, self.a);
	    },
	    "EI" => {
		self.ime = true;
	    },
	    "DI" => {
		self.ime = false;
	    },
	    "HLT" => { //todo use actual behavior
		return 0;
	    },
	    "NOP" | "*NOP" => {
		//nothing
	    },
	    _ =>
		todo!("Unimplemented instruction {}", instr.mnemonic),
	};

	self.cycles - oldcycles
    }
}

fn disas(pc: u16, opcode: u8, op1: u8, op2: u8, opw: u16) {
    print!("{:04X} ", pc);
    match opcode {
	0x00 => println!("NOP"),
	0x01 => println!("LXI    B, ${opw:04X}"),
	0x02 => println!("STAX   B"),
	0x03 => println!("INX    B"),
	0x04 => println!("INR    B"),
	0x05 => println!("DCR    B"),
	0x06 => println!("MVI    B, ${op1:02X}"),
	0x07 => println!("RLC"),
	0x08 => println!("NOP"),
	0x09 => println!("DAD    B"),
	0x0A => println!("LDAX   B"),
	0x0B => println!("DCX    B"),
	0x0C => println!("INR    C"),
	0x0D => println!("DCR    C"),
	0x0E => println!("MVI    C, ${op1:02X}"),
	0x0F => println!("RRC"),
	0x10 => println!("NOP"),
	0x11 => println!("LXI    D, ${opw:04X}"),
	0x12 => println!("STAX   D"),
	0x13 => println!("INX    D"),
	0x14 => println!("INR    D"),
	0x15 => println!("DCR    D"),
	0x16 => println!("MVI    D, ${op1:02X}"),
	0x17 => println!("RAL"),
	0x18 => println!("NOP"),
	0x19 => println!("DAD    D"),
	0x1A => println!("LDAX   D"),
	0x1B => println!("DCX    D"),
	0x1C => println!("INR    E"),
	0x1D => println!("DCR    E"),
	0x1E => println!("MVI    E, ${op1:02X}"),
	0x1F => println!("RAR"),
	0x20 => println!("NOP"),
	0x21 => println!("LXI    H, ${opw:04X}"),
	0x22 => println!("SHLD   ${opw:04X}"),
	0x23 => println!("INX    H"),
	0x24 => println!("INR    H"),
	0x25 => println!("DCR    H"),
	0x26 => println!("MVI    H, ${op1:02X}"),
	0x27 => println!("DAA"),
	0x28 => println!("NOP"),
	0x29 => println!("DAD    H"),
	0x2A => println!("LHLD   ${opw:04X}"),
	0x2B => println!("DCX    H"),
	0x2C => println!("INR    L"),
	0x2D => println!("DCR    L"),
	0x2E => println!("MVI    L, ${op1:02X}"),
	0x2F => println!("CMA"),
	0x30 => println!("NOP"),
	0x31 => println!("LXI    SP, ${opw:04X}"),
	0x32 => println!("STA    ${opw:04X}"),
	0x33 => println!("INX    SP"),
	0x34 => println!("INR    M"),
	0x35 => println!("DCR    M"),
	0x36 => println!("MVI    M, ${op1:02X}"),
	0x37 => println!("STC"),
	0x38 => println!("NOP"),
	0x39 => println!("DAD    SP"),
	0x3A => println!("LDA    ${opw:04X}"),
	0x3B => println!("DCX    SP"),
	0x3C => println!("INR    A"),
	0x3D => println!("DCR    A"),
	0x3E => println!("MVI    A, ${op1:02X}"),
	0x3F => println!("CMC"),
	0x40 => println!("MOV    B, B"),
	0x41 => println!("MOV    B, C"),
	0x42 => println!("MOV    B, D"),
	0x43 => println!("MOV    B, E"),
	0x44 => println!("MOV    B, H"),
	0x45 => println!("MOV    B, L"),
	0x46 => println!("MOV    B, M"),
	0x47 => println!("MOV    B, A"),
	0x48 => println!("MOV    C, B"),
	0x49 => println!("MOV    C, C"),
	0x4A => println!("MOV    C, D"),
	0x4B => println!("MOV    C, E"),
	0x4C => println!("MOV    C, H"),
	0x4D => println!("MOV    C, L"),
	0x4E => println!("MOV    C, M"),
	0x4F => println!("MOV    C, A"),
	0x50 => println!("MOV    D, B"),
	0x51 => println!("MOV    D, C"),
	0x52 => println!("MOV    D, D"),
	0x53 => println!("MOV    D, E"),
	0x54 => println!("MOV    D, H"),
	0x55 => println!("MOV    D, L"),
	0x56 => println!("MOV    D, M"),
	0x57 => println!("MOV    D, A"),
	0x58 => println!("MOV    E, B"),
	0x59 => println!("MOV    E, C"),
	0x5A => println!("MOV    E, D"),
	0x5B => println!("MOV    E, E"),
	0x5C => println!("MOV    E, H"),
	0x5D => println!("MOV    E, L"),
	0x5E => println!("MOV    E, M"),
	0x5F => println!("MOV    E, A"),
	0x60 => println!("MOV    H, B"),
	0x61 => println!("MOV    H, C"),
	0x62 => println!("MOV    H, D"),
	0x63 => println!("MOV    H, E"),
	0x64 => println!("MOV    H, H"),
	0x65 => println!("MOV    H, L"),
	0x66 => println!("MOV    H, M"),
	0x67 => println!("MOV    H, A"),
	0x68 => println!("MOV    L, B"),
	0x69 => println!("MOV    L, C"),
	0x6A => println!("MOV    L, D"),
	0x6B => println!("MOV    L, E"),
	0x6C => println!("MOV    L, H"),
	0x6D => println!("MOV    L, L"),
	0x6E => println!("MOV    L, M"),
	0x6F => println!("MOV    L, A"),
	0x70 => println!("MOV    M, B"),
	0x71 => println!("MOV    M, C"),
	0x72 => println!("MOV    M, D"),
	0x73 => println!("MOV    M, E"),
	0x74 => println!("MOV    M, H"),
	0x75 => println!("MOV    M, L"),
	0x76 => println!("HLT"),
	0x77 => println!("MOV    M, A"),
	0x78 => println!("MOV    A, B"),
	0x79 => println!("MOV    A, C"),
	0x7A => println!("MOV    A, D"),
	0x7B => println!("MOV    A, E"),
	0x7C => println!("MOV    A, H"),
	0x7D => println!("MOV    A, L"),
	0x7E => println!("MOV    A, M"),
	0x7F => println!("MOV    A, A"),
	0x80 => println!("ADD    B"),
	0x81 => println!("ADD    C"),
	0x82 => println!("ADD    D"),
	0x83 => println!("ADD    E"),
	0x84 => println!("ADD    H"),
	0x85 => println!("ADD    L"),
	0x86 => println!("ADD    M"),
	0x87 => println!("ADD    A"),
	0x88 => println!("ADC    B"),
	0x89 => println!("ADC    C"),
	0x8A => println!("ADC    D"),
	0x8B => println!("ADC    E"),
	0x8C => println!("ADC    H"),
	0x8D => println!("ADC    L"),
	0x8E => println!("ADC    M"),
	0x8F => println!("ADC    A"),
	0x90 => println!("SUB    B"),
	0x91 => println!("SUB    C"),
	0x92 => println!("SUB    D"),
	0x93 => println!("SUB    E"),
	0x94 => println!("SUB    H"),
	0x95 => println!("SUB    L"),
	0x96 => println!("SUB    M"),
	0x97 => println!("SUB    A"),
	0x98 => println!("SBB    B"),
	0x99 => println!("SBB    C"),
	0x9A => println!("SBB    D"),
	0x9B => println!("SBB    E"),
	0x9C => println!("SBB    H"),
	0x9D => println!("SBB    L"),
	0x9E => println!("SBB    M"),
	0x9F => println!("SBB    A"),
	0xA0 => println!("ANA    B"),
	0xA1 => println!("ANA    C"),
	0xA2 => println!("ANA    D"),
	0xA3 => println!("ANA    E"),
	0xA4 => println!("ANA    H"),
	0xA5 => println!("ANA    L"),
	0xA6 => println!("ANA    M"),
	0xA7 => println!("ANA    A"),
	0xA8 => println!("XRA    B"),
	0xA9 => println!("XRA    C"),
	0xAA => println!("XRA    D"),
	0xAB => println!("XRA    E"),
	0xAC => println!("XRA    H"),
	0xAD => println!("XRA    L"),
	0xAE => println!("XRA    M"),
	0xAF => println!("XRA    A"),
	0xB0 => println!("ORA    B"),
	0xB1 => println!("ORA    C"),
	0xB2 => println!("ORA    D"),
	0xB3 => println!("ORA    E"),
	0xB4 => println!("ORA    H"),
	0xB5 => println!("ORA    L"),
	0xB6 => println!("ORA    M"),
	0xB7 => println!("ORA    A"),
	0xB8 => println!("CMP    B"),
	0xB9 => println!("CMP    C"),
	0xBA => println!("CMP    D"),
	0xBB => println!("CMP    E"),
	0xBC => println!("CMP    H"),
	0xBD => println!("CMP    L"),
	0xBE => println!("CMP    M"),
	0xBF => println!("CMP    A"),
	0xC0 => println!("RNZ"),
	0xC1 => println!("POP    B"),
	0xC2 => println!("JNZ    ${opw:04X}"),
	0xC3 => println!("JMP    ${opw:04X}"),
	0xC4 => println!("CNZ    ${opw:04X}"),
	0xC5 => println!("PUSH   B"),
	0xC6 => println!("ADI    ${op1:02X}"),
	0xC7 => println!("RST    0"),
	0xC8 => println!("RZ"),
	0xC9 => println!("RET"),
	0xCA => println!("JZ     ${opw:04X}"),
	0xCB => println!("JMP    ${opw:04X}"),
	0xCC => println!("CZ     ${opw:04X}"),
	0xCD => println!("CALL   ${opw:04X}"),
	0xCE => println!("ACI    ${op1:02X}"),
	0xCF => println!("RST    1"),
	0xD0 => println!("RNC"),
	0xD1 => println!("POP    D"),
	0xD2 => println!("JNC    ${opw:04X}"),
	0xD3 => println!("OUT    ${op1:02X}"),
	0xD4 => println!("CNC    ${opw:04X}"),
	0xD5 => println!("PUSH   D"),
	0xD6 => println!("SUI    ${op1:02X}"),
	0xD7 => println!("RST    2"),
	0xD8 => println!("RC"),
	0xD9 => println!("RET"),
	0xDA => println!("JC     ${opw:04X}"),
	0xDB => println!("IN     ${op1:02X}"),
	0xDC => println!("CC     ${opw:04X}"),
	0xDD => println!("CALL   ${opw:04X}"),
	0xDE => println!("SBI    ${op1:02X}"),
	0xDF => println!("RST    3"),
	0xE0 => println!("RPO"),
	0xE1 => println!("POP    H"),
	0xE2 => println!("JPO    ${opw:04X}"),
	0xE3 => println!("XTHL"),
	0xE4 => println!("CPO    ${opw:04X}"),
	0xE5 => println!("PUSH   H"),
	0xE6 => println!("ANI    ${op1:02X}"),
	0xE7 => println!("RST    4"),
	0xE8 => println!("RPE"),
	0xE9 => println!("PCHL"),
	0xEA => println!("JPE    ${opw:04X}"),
	0xEB => println!("XCHG"),
	0xEC => println!("CPE    ${opw:04X}"),
	0xED => println!("CALL   ${opw:04X}"),
	0xEE => println!("XRI    ${op1:02X}"),
	0xEF => println!("RST    5"),
	0xF0 => println!("RP"),
	0xF1 => println!("POP    PSW"),
	0xF2 => println!("JP     ${opw:04X}"),
	0xF3 => println!("DI"),
	0xF4 => println!("CP     ${opw:04X}"),
	0xF5 => println!("PUSH   PSW"),
	0xF6 => println!("ORI    ${op1:02X}"),
	0xF7 => println!("RST    6"),
	0xF8 => println!("RM"),
	0xF9 => println!("SPHL"),
	0xFA => println!("JM     ${opw:04X}"),
	0xFB => println!("EI"),
	0xFC => println!("CM     ${opw:04X}"),
	0xFD => println!("CALL   ${opw:04X}"),
	0xFE => println!("CPI    ${op1:02X}"),
	0xFF => println!("RST    7"),
    };
}
