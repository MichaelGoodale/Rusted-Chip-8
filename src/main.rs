extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::EventPump;
use sdl2::rect::{Rect};

use std::{thread, time};   

const MEMORY_SIZE: usize = 4096;
const REGISTER_SIZE: usize = 16;
const WHITE:Color = Color::RGB(255, 255, 255);
const BLACK:Color = Color::RGB(0, 0, 0);
const WINDOW_SCALE: u32 = 4;
fn main() {
	let mut ram:[i8; MEMORY_SIZE] = [0; MEMORY_SIZE];
	let mut registers:[i8; REGISTER_SIZE] = [0; REGISTER_SIZE];
	let mut I:i16;


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

