mod cpu;
mod bus;

use std::env;
use std::io;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let mut cpu = cpu::Cpu::new(true);
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
    let width = 256;
    let height = 224;
    let win = video.window("Space Invaders", width, height)
	.position_centered()
	.opengl()
	.build()
	.unwrap();
    let mut canv = win.into_canvas().build().unwrap();
    let tex_create = canv.texture_creator();
    let mut tex = tex_create
	.create_texture_streaming(PixelFormatEnum::RGB24, 256, 224)
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
	//let _ = stdin.read(&mut [0u8]).unwrap();
    }
}
