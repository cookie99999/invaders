mod cpu;
mod bus;

use std::env;

fn main() {
    let mut cpu = cpu::Cpu::new(true);

    let path = env::args().nth(1).expect("Usage: invaders <path>");
    let buf: Vec<u8> = std::fs::read(path).unwrap();
    let stub_buf: Vec<u8> = std::fs::read("cpmstub.bin").unwrap();
    cpu.bus.load_bin(0, &stub_buf);
    cpu.bus.load_bin(0x100, &buf);
    cpu.reset();
    cpu.pc = 0x100;
    'running: loop {
	if cpu.step() == 0 {
	    break 'running;
	}
    }
}
