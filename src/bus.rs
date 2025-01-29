const ROM_START: u16 = 0x0000;
const ROM_END: u16 = 0x1fff;
const RAM_START: u16 = 0x2000;
const RAM_END: u16 = 0x23ff;
const VRAM_START: u16 = 0x2400;
const VRAM_END: u16 = 0x3fff;

pub trait Bus {
    fn read_byte(&mut self, addr: u16) -> u8;
    fn read_word(&mut self, addr: u16) -> u16;
    fn write_byte(&mut self, addr: u16, data: u8);
    fn write_word(&mut self, addr: u16, data: u16);
    fn read_io_byte(&mut self, port: u8) -> u8;
    fn write_io_byte(&mut self, port: u8, data: u8);
    fn load_bin(&mut self, offs: usize, buf: &[u8]);
}

pub struct InvBus {
    rom: [u8; 0x2000],
    ram: [u8; 0x400],
    pub vram: [u8; 0x1c00],
    cycles: usize,
    shift_amt: u8,
    watchdog: u8,
    shift_reg: u16,
    pub irq: bool,
    pub irq_vec: u8,
    pub credit: bool,
    pub p1_start: bool,
    pub p2_start: bool,
    pub p1_fire: bool,
    pub p2_fire: bool,
    pub p1_left: bool,
    pub p2_left: bool,
    pub p1_right: bool,
    pub p2_right: bool,
    pub dip: u8,
    half: bool,
    pub vblank: bool,
}

pub struct CpmBus {
    ram: [u8; 0x10000],
}

impl Bus for CpmBus {
    fn read_byte(&mut self, addr: u16) -> u8 {
	self.ram[addr as usize]
    }

    fn read_word(&mut self, addr: u16) -> u16 {
	let result: u16 = self.read_byte(addr) as u16 | ((self.read_byte(addr + 1) as u16) << 8);
	result
    }

    fn write_byte(&mut self, addr: u16, data: u8) {
	self.ram[addr as usize] = data;
    }

    fn write_word(&mut self, addr: u16, data: u16) {
	self.write_byte(addr, (data >> 8) as u8);
	self.write_byte(addr + 1, (data & 0x00ff) as u8);
    }

    fn read_io_byte(&mut self, port: u8) -> u8 {
	match port {
	    _ =>
		todo!("unhandled io port read {port:02x}"),
	}
    }

    fn write_io_byte(&mut self, port: u8, data: u8) {
	match port {
	    0xaa => {
		print!("{}", data as char);
	    },
	    0xff => panic!("warm booted"),
	    _ =>
		todo!("unhandled io port write {port:02x}"),
	};
    }

    fn load_bin(&mut self, offs: usize, buf: &[u8]) {
	for i in 0..buf.len() {
	    self.ram[offs + i] = buf[i];
	}
    }
}

impl Bus for InvBus {
    fn read_byte(&mut self, addr: u16) -> u8 {
	match addr {
	    ROM_START ..= ROM_END =>
		self.rom[addr as usize],
	    RAM_START ..= RAM_END =>
		self.ram[(addr - RAM_START) as usize],
	    VRAM_START ..= VRAM_END =>
		self.vram[(addr - VRAM_START) as usize],
	    _ =>
		self.ram[(addr % (RAM_END - RAM_START)) as usize],
	}
    }

    fn read_word(&mut self, addr: u16) -> u16 {
	let result: u16 = self.read_byte(addr) as u16 | ((self.read_byte(addr + 1) as u16) << 8);
	result
    }

    fn write_byte(&mut self, addr: u16, data: u8) {
	match addr {
	    ROM_START ..= ROM_END =>
		println!("attempted write to rom at {addr:04X}"),
	    RAM_START ..= RAM_END =>
		self.ram[(addr - RAM_START) as usize] = data,
	    VRAM_START ..= VRAM_END =>
		self.vram[(addr - VRAM_START) as usize] = data,
	    _ =>
		self.ram[(addr % (RAM_END - RAM_START)) as usize] = data,
	};
    }

    fn write_word(&mut self, addr: u16, data: u16) {
	self.write_byte(addr, (data >> 8) as u8);
	self.write_byte(addr + 1, (data & 0x00ff) as u8);
    }

    fn read_io_byte(&mut self, port: u8) -> u8 {
	match port {
	    0 => ((self.dip >> 4) & 1) | 0b01110000,
	    1 => {
		((self.p1_right as u8) << 6) | ((self.p1_left as u8) << 5) |
		((self.p1_fire as u8) << 4) | 8 | ((self.p1_start as u8) << 2) |
		((self.p2_start as u8) << 1) | self.credit as u8
	    },
	    2 => {
		(self.dip & 0x80) | ((self.p2_right as u8) << 6) |
		((self.p2_left as u8) << 5) | ((self.p2_fire as u8) << 4) |
		(((self.dip >> 6) & 1) << 3) | (((self.dip >> 5) & 1) << 1) |
		((self.dip >> 3) & 1)
	    },
	    3 => ((self.shift_reg << self.shift_amt) >> 8) as u8,
	    6 => 0, //watchdog timer
	    _ =>
		todo!("unhandled io port read {port:02x}"),
	}
    }

    fn write_io_byte(&mut self, port: u8, data: u8) {
	match port {
	    2 => self.shift_amt = data & 7,
	    3 => println!("played sound 3.{data}"),
	    4 => {
		let tmp = (self.shift_reg >> 8) & 0xff;
		self.shift_reg = (data as u16) << 8 | tmp;
	    },
	    5 => println!("played sound 5.{data}"),
	    6 => {}, //normally a watchdog access resets a timer, that if allowed to count down
	    //would reset the hardware. this probably only happens from hardware failure
	    //in the case of the real machine, or improper emulation/corrupt rom dump,
	    //so it isnt necessary to emulate accurately
	    _ =>
		todo!("unhandled io port write {port:02x}"),
	};
    }

    fn load_bin(&mut self, offs: usize, buf: &[u8]) {
	for i in 0..buf.len() {
	    self.rom[offs + i] = buf[i];
	}
    }
}

impl InvBus {
    pub fn new() -> Self {
	InvBus {
	    rom: [0; 0x2000],
	    ram: [0; 0x400],
	    vram: [0; 0x1c00],
	    cycles: 0,
	    shift_amt: 0,
	    watchdog: 0,
	    shift_reg: 0,
	    irq: false,
	    irq_vec: 0,
	    credit: false,
	    p1_start: false,
	    p2_start: false,
	    p1_fire: false,
	    p2_fire: false,
	    p1_left: false,
	    p2_left: false,
	    p1_right: false,
	    p2_right: false,
	    dip: 0,
	    half: true,
	    vblank: false,
	}
    }

    pub fn step(&mut self, cyc: usize) {
	//count the time until interrupts
	self.cycles += cyc;
	if self.cycles >= 16667 / 2 && self.half { //half frame
	    self.irq = true;
	    self.irq_vec = 0xcf; //RST 8
	    self.cycles -= 16667 / 2;
	    self.half = false;
	}
	
	if self.cycles >= 16667 { //~1 frame
	    self.irq = true;
	    self.irq_vec = 0xd7; //RST 10
	    self.cycles -= 16667;
	    self.half = true;
	    self.vblank = true;
	}
    }
}

impl CpmBus {
    pub fn new() -> Self {
	CpmBus {
	    ram: [0; 0x10000],
	}
    }
}
