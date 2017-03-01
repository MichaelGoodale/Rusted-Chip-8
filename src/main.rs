extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::EventPump;
use sdl2::rect::{Rect};

use std::{thread, time};   

const WHITE:Color = Color::RGB(255, 255, 255);
const BLACK:Color = Color::RGB(0, 0, 0);
const WINDOW_SCALE: u32 = 20;

struct cpu {
	ram:[u8; 4096],
	v:[u8; 16],
	opcode:u16,
	i:u16,
	pc:u8,

	delay_timer:u8,
	sound_timer:u8,
}

impl cpu {	
	fn init() -> cpu {
		let mut c = cpu {
			ram: [0; 4096],
			v: [0; 16],
			opcode:0,
			i:0,
			pc:0,
			
			delay_timer:0,
			sound_timer:0,
		};
		
		c.ram[0:80] =
		{ 
		  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
		  0x20, 0x60, 0x20, 0x20, 0x70, // 1
		  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
		  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
		  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
		  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
		  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
		  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
		  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
		  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
		  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
		  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
		  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
		  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
		  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
		  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
		};
	}
}

fn main() {
	let mut c = cpu::init();
	graphics_loop();
}

fn graphics_loop(){
	let (mut r, mut e) = init_graphics();
	'event : loop {
		for event in e.poll_iter() {
		    match event {
			Event::Quit{..} => break 'event,
			Event::KeyDown {keycode: Some(keycode), ..} => {
			    if keycode == Keycode::Escape {
				break 'event
			    }
			}
			_ => continue
		    }
		}

		//MAIN LOOP IS HERE
		r.set_draw_color(WHITE);
		r.clear();
		r.present();
		thread::sleep(time::Duration::from_millis(10));
	}
}

fn init_graphics<'a>() -> (Renderer<'a>, EventPump) {
	let ctx = sdl2::init().unwrap();
	let video_ctx = ctx.video().unwrap();
	
	let window = video_ctx.window("Chip 8 Emu", 64*WINDOW_SCALE, 32*WINDOW_SCALE)
		.position_centered()
		.opengl()
		.build()
		.unwrap();
	
	let mut renderer = window.renderer().build().unwrap();
	renderer.set_scale(WINDOW_SCALE as f32, WINDOW_SCALE as f32);
	renderer.set_draw_color(Color::RGB(255, 0, 0));
	renderer.clear();
	renderer.present();
	let mut events = ctx.event_pump().unwrap();

	(renderer, events)
}

