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
    inp0: u8,
    inp1: u8,
    inp2: u8,
    shift_in: u8,
    shift_amt: u8,
    sound1: u8,
    shift_data: u8,
    sound2: u8,
    watchdog: u8,
    shift_reg: u16,
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
	    0xff => println!("warm booted"),
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
		self.ram[(addr % RAM_START) as usize],
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
		self.ram[(addr % RAM_START) as usize] = data,
	};
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
	    _ =>
		todo!("unhandled io port write {port:02x}"),
	};
    }

    fn load_bin(&mut self, offs: usize, buf: &[u8]) {
	for i in 0..buf.len() {
	    self.write_byte(offs as u16 + i as u16, buf[i]);
	}
    }
}

impl InvBus {
    pub fn new() -> Self {
	InvBus {
	    rom: [0; 0x2000],
	    ram: [0; 0x400],
	    vram: [0; 0x1c00],
	    inp0: 0,
	    inp1: 0,
	    inp2: 0,
	    shift_in: 0,
	    shift_amt: 0,
	    sound1: 0,
	    shift_data: 0,
	    sound2: 0,
	    watchdog: 0,
	    shift_reg: 0,
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
