extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::EventPump;
use sdl2::rect::Point;

use std::{thread, time, env};   
mod cpu;

const WHITE:Color = Color::RGB(255, 255, 255);
const BLACK:Color = Color::RGB(0, 0, 0);
const WINDOW_SCALE: u32 = 20;

fn main() {
	let args: Vec<_> = env::args().collect();
	if args.len() <= 1 {
		println!("You must give a game name!");
	}
	let mut c = cpu::Cpu::new();
	let (mut r, mut e) = init_graphics();
	c.load_rom(&args[1].to_string());
	'event : loop {
		for event in e.poll_iter() {
		    match event {
			Event::Quit{..} => break 'event,
			Event::KeyDown {keycode: Some(keycode), ..} => {
				if keycode == Keycode::Escape {
					break 'event
				}else{
					let (key_num, valid_key) = match_key(keycode);
					if valid_key {
						c.press_key(key_num);
					}else{
						continue;
					}
				}
			},
			Event::KeyUp {keycode : Some(keycode), ..} => {
				let (key_num, valid_key) = match_key(keycode);
				if valid_key {
					c.release_key(key_num);
				}else{
					continue;
				}
			},
			_ => continue
		    }
		}
		if(c.do_cycle()) {
			'inputwait: for key in e.wait_iter(){
				match key {
					Event::Quit{..} => break 'event,
					Event::KeyDown {keycode: Some(keycode), ..} => {
						if keycode == Keycode::Escape {
							break 'event
						}else{
							let (key_num, valid_key) = match_key(keycode);
							if valid_key {
								c.press_key(key_num);
							}else{
								continue;
							}
						}
					}
					_ => continue
				}
			}
		}
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
		thread::sleep(time::Duration::from_millis(17));//17));
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

fn match_key(key: Keycode) -> (u8, bool) {
	match key {
		Keycode::Num1 => (0x1, true),
		Keycode::Num2 => (0x2, true),
		Keycode::Num3 => (0x3, true),
		Keycode::Num4 => (0xC, true),
		Keycode::Q => (0x4, true),
		Keycode::W => (0x5, true),
		Keycode::E => (0x6, true),
		Keycode::R => (0xD, true),
		Keycode::A => (0x7, true),
		Keycode::S => (0x8, true),
		Keycode::D => (0x9, true),
		Keycode::F => (0xE, true),
		Keycode::Z => (0xA, true),
		Keycode::X => (0x0, true),
		Keycode::C => (0xB, true),
		Keycode::V => (0xF, true),
		_ => (0, false),

	}
}
