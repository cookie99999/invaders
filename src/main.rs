mod cpu;
mod bus;

use std::env;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut cpu = cpu::Cpu::new(true);
    let mut stdin = io::stdin();

    let path = env::args().nth(1).expect("Usage: invaders <path>");
    let buf: Vec<u8> = std::fs::read(path).unwrap();
    let stub_buf: Vec<u8> = std::fs::read("cpmstub.bin").unwrap();
    cpu.bus.load_bin(0xdc00, &stub_buf);
    cpu.bus.write_byte(5, 0xc3);
    cpu.bus.write_word(6, 0x00dc); //jmp $dc00
    cpu.bus.load_bin(0x100, &buf);
    cpu.reset();
    cpu.pc = 0x100;
    'running: loop {
	if cpu.step() == 0 || cpu.pc == 0 {
	    break 'running;
	}
	//let _ = stdin.read(&mut [0u8]).unwrap();
    }
}
