extern crate rand;
use std::fs::File;
use std::env::current_dir;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;

pub struct Cpu {
	ram:[u8; 4096],
	v:[u8; 16],
	opcode:u16,
	i:u16,
	pc:u16,
	
	stack:[u16;16],
	sp:u8,
	
	delay_timer:u8,
	sound_timer:u8,

	keys:[bool; 16],
	gfx:[[u8;32];64],
	draw_flag:bool,
}

impl Cpu {
	fn fonts() -> [u8; 80] {
		[ 
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
		]
	}
	pub fn load_rom(&mut self, game: &str){
		self.reset();
		//LOAD ROM
		let path = Path::new("/home/michael/rust_projects/chip8_emu/assets/CONNECT4");

		let display = path.display();
		let mut file = match File::open(&path) {
			Err(why) => panic!("couldn't open {}: {}", display,
								   why.description()),
			Ok(file) => file,
		};
		let mut ram_ptr = 0x200;
		for byte in file.bytes() {
			self.ram[ram_ptr] = byte.unwrap();
			ram_ptr+=1;
		}
		
	}	
	pub fn reset(&mut self) {
		self.ram = [0; 4096];
		self.v = [0; 16];
		self.opcode = 0;
		self.i = 0;
		self.pc = 0x200;
		
		self.stack = [0;16]; 
		self.sp = 0;

		self.delay_timer=0;
		self.sound_timer=0;
		for i in 0 .. 80 {
			self.ram[i]=Cpu::fonts()[i];
		}
		self.keys = [false; 16];
		self.gfx = [[0;32];64];
		self.draw_flag=false;
	}

	pub fn new() -> Cpu {
		let mut c = Cpu {
			ram: [0; 4096],
			v: [0; 16],
			opcode: 0,
			i: 0,
			pc: 0x200,

			stack: [0;16],
			sp: 0,

			delay_timer: 0,
			sound_timer: 0,
			keys: [false; 16],
			gfx: [[0;32];64],
			draw_flag: false,
		};
		c.reset();
		c
	}
	
	pub fn get_gfx(&self) -> [[u8;32];64]{
		self.gfx
	}
	pub fn draw_gfx(&self) -> bool {
		self.draw_flag	
	}	
	pub fn print_ram(&self, min:usize, max:usize){
		for i in min .. max {
			println!("{}",self.ram[i]);
		}
	}

	pub fn do_cycle(&mut self){
		let opcode:u16 = self.get_opcode();
		println!("Opcode is {:X}, at pc={:X}, with i = {:X}",opcode, self.pc, self.i);
		self.draw_flag=false;
		let addr = opcode & 0xFFF;
		let nibble = opcode & 0xF;
		let x = (opcode & 0xF00) >> 8;
		let y = (opcode & 0xF0) >> 4;
		let kk  = (opcode & 0xFF) as u8;
		//Match top four bits
		match (opcode & 0xF000)>>12  {
			0 => match opcode {
				//CLS
				0x00E0 => {
					self.draw_flag = true;
					self.gfx = [[0; 32];64];
				},
				//RET
				0x00EE => {
					self.pc = self.stack[self.sp as usize];
					self.sp -= 1;
				},
				_ => println!("SYS addr, ignoring"), 
			},
			1 => self.pc = opcode & 0xFFF, //JP addr
			2 => {
				//CALL addr
				self.sp += 1;
				self.stack[self.sp as usize] = self.pc;
				self.pc = opcode & 0xFFF;		
			},
			3 => {
				//SE Vx, byte
				if self.v[x as usize] == kk {
					self.pc += 2;
				}
			},
			4 => {
				//SNE Vx, byte
				if self.v[x as usize] != kk {
					self.pc += 2;
				}
			},
			5 if nibble == 0 => {
				//SE Vx, Vy
				if self.v[x as usize] == self.v[y as usize] {
					self.pc += 2;
				}	
			},
			//LD Vx, byte
			6 => self.v[x as usize] = kk,
			//ADD Vx, byte
			7 => self.v[x as usize] = self.v[x as usize].overflowing_add(kk).0,
			8 => match nibble {
				//LD Vx, Vy
				0 => self.v[x as usize] = self.v[y as usize],
				//OR Vx, Vy
				1 => self.v[x as usize] = self.v[x as usize] | self.v[y as usize],
				//AND Vx, Vy
				2 => self.v[x as usize] = self.v[x as usize] & self.v[y as usize],
				//XOR Vx, Vy
				3 => self.v[x as usize] = self.v[x as usize] ^ self.v[y as usize],
				4 => {
					//ADD Vx, Vy
					//Gets a tuple with addition and whether we set the carry flag.
					let ans = self.v[x as usize].overflowing_add(self.v[y as usize]);
					self.v[x as usize] = ans.0;
					self.set_vf(ans.1);
				},
				5 => {
					//SUB Vx, Vy
					let boolean = self.v[x as usize] > self.v[y as usize];
					self.set_vf(boolean);
					self.v[x as usize] -= self.v[y as usize];
						
				},
				6 => {
					//SHR Vx {, Vy}
					let boolean = self.v[x as usize] & 1 == 1;
					self.set_vf(boolean);
					self.v[x as usize] /= 2;
				}, 
				7 => {
					//SUBN Vx, Vy
					let boolean = self.v[y as usize] > self.v[x as usize];
					self.set_vf(boolean);
					self.v[x as usize] = self.v[y as usize] - self.v[x as usize];
						
				},
				0xE => {
					//SHR Vx {, Vy}
					let boolean = self.v[x as usize] & 1 == 1;
					self.set_vf(boolean);
					self.v[x as usize] = self.v[x as usize].overflowing_mul(2u8).0;
				},
				_ => println!("Unregonised opcode"),
			},
			//SNE Vx, Vy
			9 => if self.v[x as usize] != self.v[y as usize] { self.pc += 2 },
			//LD i, addr
			0xA => self.i = addr,
			//JP V0, addr
			0xB => self.pc = addr+(self.v[0] as u16),
			//RND Vx, byte
			0xC => {
				let rand_byte = rand::random::<u8>();
				self.v[x as usize] = rand_byte & kk;
			},
			//DRW Vx, Vy, nibble
			0xD => {
				self.v[15] = 0; //Set vf
				let (Vx, Vy) = (self.v[x as usize] as usize, self.v[y as usize] as usize);
				for i in 0 .. nibble as usize {
					let sprite_level = self.ram[i + (self.i as usize)];
					print!("Sprite level {} is {:X}", i, sprite_level);
					for j in 0 .. 8 {
						//Get the bit for each given pixel.
						let pixel = (sprite_level >> (7-j)) & 1;
						let set = self.gfx[(Vx+j)%64][(Vy+i) % 32];
						self.gfx[(Vx+j)%64][(Vy+i) % 32] ^= pixel;
						if set == 1 && self.gfx[(Vx+j)%64][(Vy+i) % 32] == 0 {
							self.v[15]=1;
						}
						print!("{}", pixel);
					}
					println!("");
				}
				self.draw_flag = true;
			},
			//SKP Vx
			0xE if kk == 0x9E => if self.keys[x as usize] { self.pc += 2 },
			//SKNP Vx
			0xE if kk == 0xA1 => if !self.keys[x as usize] { self.pc += 2 },
			
			0xF => match kk {
				//LD Vx, DT
				0x07 => self.v[x as usize] = self.delay_timer,
				//LD Vx, K
				0x0A => println!("boo"), //TODO wait until key press
				//LD DT, Vx
				0x15 => self.delay_timer = self.v[x as usize],
				//LD ST, Vx
				0x18 => self.sound_timer = self.v[x as usize],
				//ADD I, Vx
				0x1E => self.i = self.i + (self.v[x as usize] as u16),
				//LD F, Vx
				0x29 => self.i = 5 * self.v[x as usize] as u16,//5 since each sprite has a height of 5.
				//LD B, Vx
				0x33 => {
					let mut bcd = self.v[x as usize];
					//Puts hundreds in ram[i], tens in ram[i+1] etc.
					for i in (0 .. 3).rev() {
						self.ram[i + (self.i as usize)] = bcd % 10;
						bcd /= 10;
					}
				},
				//LD [I], Vx
				0x55 => {	
					for i in 0 .. (x as usize) + 1 {
						self.ram[i + (self.i as usize)] = self.v[i];
					}
					self.i = self.i+(x as u16)+1;
				},
				//LD Vx, [I]
				0x65 => {
					for i in 0 .. (x as usize) + 1 {
						self.v[i] = self.ram[i + (self.i as usize)];
					}
					self.i = self.i+(x as u16)+1;
				},
				_ => println!("Unrecognised opcode"),
			},	
		_ => println!("Unrecognised opcode"),
		}

		self.pc += 2;
		
		//Decrement timers
		if self.delay_timer > 0{
			self.delay_timer -= 1;
		}
		if self.sound_timer > 0 {
			//Do audio
			println!("PRETEND THERE WAS SOUND");
			self.sound_timer -= 1;
		}

	}

	pub fn set_vf(&mut self, x:bool){
		if x {
			self.v[15] = 1u8;
		}else{
			self.v[15] = 0u8;
		}
		
	}

	pub fn print_registry(&self) {
		for i in 0 .. 16 {
			println!("V[{}] = {}",i, self.v[i]);
		}
	}
	pub fn do_opcode(&mut self, opcode: u16) {
		println!("{:X} + {:X} = {:X}", ((opcode & 0xFF00) >> 8),opcode & 0xFF, opcode);
		self.ram[(self.pc+1) as usize] = (opcode & 0xFF) as u8;
		self.ram[self.pc as usize] = ((opcode & 0xFF00) >> 8) as u8;
		self.do_cycle();
	}	
	pub fn press_key(&mut self, key: u16) {
		self.keys[key as usize] = true;
	}

	pub fn release_key(&mut self, key: u16) {
		self.keys[key as usize] = false;
	}
	
	fn get_opcode(&self) -> u16{
		(self.ram[self.pc as usize] as u16) << 8 | (self.ram[(self.pc as usize) + 1] as u16)
	}	
}
