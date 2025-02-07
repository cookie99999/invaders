mod cpu;
mod bus;

use crate::bus::Bus;
use std::env;
use std::thread;
use std::time;
use std::io;
use std::io::prelude::*;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};

fn draw(bus: &mut bus::InvBus, tex: &mut sdl2::render::Texture) {
    tex.with_lock(None, |buf: &mut [u8], pitch: usize| {
	for x in (0..256).step_by(8) {
	    for y in 0..224 {
		for b in (0..8).rev() {
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
    let mut cpu = cpu::Cpu::new();
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
    
    let _audio = context.audio().unwrap();
    sdl2::mixer::open_audio(44_100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024).unwrap();
    let _mixer_context = sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG)
	.unwrap();
    sdl2::mixer::allocate_channels(4);
    let sfx_chunks: [sdl2::mixer::Chunk; 9] = [ sdl2::mixer::Chunk::from_file("sfx/0.wav").unwrap(),
						sdl2::mixer::Chunk::from_file("sfx/1.wav").unwrap(),
						sdl2::mixer::Chunk::from_file("sfx/3.wav").unwrap(),
						sdl2::mixer::Chunk::from_file("sfx/4.wav").unwrap(),
						sdl2::mixer::Chunk::from_file("sfx/5.wav").unwrap(),
						sdl2::mixer::Chunk::from_file("sfx/6.wav").unwrap(),
						sdl2::mixer::Chunk::from_file("sfx/7.wav").unwrap(),
						sdl2::mixer::Chunk::from_file("sfx/8.wav").unwrap(),
						sdl2::mixer::Chunk::from_file("sfx/9.wav").unwrap(),
    ];
    
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

    let cycle_time = time::Duration::from_nanos(50); //2mhz clock
    'running: loop {
	let now = time::Instant::now();
	for e in event_pump.poll_iter() {
	    match e {
		Event::Quit {..} |
		Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
		    break 'running;
		},
		Event::KeyDown { keycode: Some(kc), .. } => {
		    match kc {
			Keycode::LEFT => cpu.bus.p1_left = true,
			Keycode::RIGHT => cpu.bus.p1_right = true,
			Keycode::LCTRL => cpu.bus.p1_fire = true,
			Keycode::C => cpu.bus.credit = true,
			Keycode::RETURN => cpu.bus.p1_start = true,
			_ => {},
		    };
		},
		Event::KeyUp { keycode: Some(kc), .. } => {
		    match kc {
			Keycode::LEFT => cpu.bus.p1_left = false,
			Keycode::RIGHT => cpu.bus.p1_right = false,
			Keycode::LCTRL => cpu.bus.p1_fire = false,
			Keycode::C => cpu.bus.credit = false,
			Keycode::RETURN => cpu.bus.p1_start = false,
			_ => {},
		    };
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

	    for i in 0..9 {
		if cpu.bus.sfx[i] {
		    sdl2::mixer::Channel::all().halt();
		    sdl2::mixer::Channel::all().play(&sfx_chunks[i], 0).unwrap();
		    cpu.bus.sfx[i] = false;
		}
	    }
	}
	
	//let _ = stdin.read(&mut [0u8]).unwrap();
	let elapsed = now.elapsed();
	let target = cycle_time.saturating_mul(cyc as u32);
	//println!("took {} target is {}",
	//	 elapsed.as_nanos(),
	//	 target.as_nanos());
	if elapsed < target {
	    thread::sleep(target.saturating_sub(elapsed));
	}
    }
}
