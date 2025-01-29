mod cpu;
mod bus;

use crate::bus::Bus;
use std::env;
use std::io;
use std::io::prelude::*;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn draw(bus: &mut bus::InvBus, tex: &mut sdl2::render::Texture) {
    tex.with_lock(None, |buf: &mut [u8], pitch: usize| {
	for x in (0..256).step_by(8) {
	    for y in 0..224 {
		for (i, b) in (0..8).rev().enumerate() {
		    let bufx: usize = y;
		    let bufy: usize = (-(x as i16 + b as i16) + 256 - 1) as usize;
		    let buf_offs: usize = bufy * pitch + bufx;
		    let offs: usize = (y * (256 / 8)) + (x / 8);
		    let px: u8 = (bus.vram[offs] >> b) & 1;
		    let c: u8 = match px {
			0 => 0x00,
			_ => 0xff,
		    };
		    buf[buf_offs] = c;
		}
	    }
	}
    }).unwrap();
}
	    
fn main() {
    let mut cpu = cpu::Cpu::new(false);
    let mut stdin = io::stdin();

    let path = env::args().nth(1).expect("Usage: invaders <path>");
    let buf: Vec<u8> = std::fs::read(path).unwrap();
    //let stub_buf: Vec<u8> = std::fs::read("cpmstub.bin").unwrap();
    //cpu.bus.load_bin(0xdc00, &stub_buf);
    //cpu.bus.write_byte(5, 0xc3);
    //cpu.bus.write_word(6, 0x00dc); //jmp $dc00
    cpu.bus.load_bin(0, &buf);
    cpu.reset();
    //cpu.pc = 0x100;

    let context = sdl2::init().unwrap();
    let mut event_pump = context.event_pump().unwrap();
    let video = context.video().unwrap();
    let width = 224;
    let height = 256;
    let win = video.window("Space Invaders", width, height)
	.position_centered()
	.opengl()
	.build()
	.unwrap();
    let mut canv = win.into_canvas().build().unwrap();
    let tex_create = canv.texture_creator();
    let mut tex = tex_create
	.create_texture_streaming(PixelFormatEnum::RGB332, 224, 256)
	.unwrap();
    canv.clear();
    canv.present();
    
    'running: loop {
	for e in event_pump.poll_iter() {
	    match e {
		Event::Quit {..} |
		Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
		    break 'running;
		},
		_ => {},
	    }
	}
	
	let cyc = cpu.step();
	if cyc == 0 {
	    break 'running;
	}
	cpu.bus.step(cyc);
	if cpu.bus.vblank {
	    draw(&mut cpu.bus, &mut tex);
	    cpu.bus.vblank = false;
	    canv.copy(&tex, None, None).unwrap();
	    canv.present();
	}
	//if cpu.cycles > 600000 {
	//    let _ = stdin.read(&mut [0u8]).unwrap();
	//}
    }
}
