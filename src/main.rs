extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::EventPump;
use sdl2::rect::Point;

use std::{thread, time};   
mod cpu;

const WHITE:Color = Color::RGB(255, 255, 255);
const BLACK:Color = Color::RGB(0, 0, 0);
const WINDOW_SCALE: u32 = 20;

fn main() {
	let mut c = cpu::Cpu::new();
	//c.load_rom("PONG");
	let (mut r, mut e) = init_graphics();
	c.load_rom();
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
		c.do_cycle();
		/*
		c.do_opcode(0x00E0);
		c.do_opcode(0xC10F);
		c.do_opcode(0xF129);
		c.do_opcode(0xD005);
		*/
		//MAIN LOOP IS HERE
		if c.draw_gfx(){
			let gfx:[[u8; 32]; 64] = c.get_gfx();
			r.set_draw_color(BLACK);
			r.clear();
			r.set_draw_color(WHITE);
			for i in 0 .. 64 {
				for j in 0 .. 32 {
					if gfx[i][j]==1 {
						r.draw_point(Point::new(i as i32, j as i32));
					}
				}
			}
			r.present();
		};
		//Sleep for roughly 1/60 of a second
		thread::sleep(time::Duration::from_millis(1));//17));
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

