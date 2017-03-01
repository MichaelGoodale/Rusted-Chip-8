extern crate rand;

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
	gfx:[[bool;32];64],
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
	
	pub fn reset(&mut self) {
		self.ram = [0; 4096];
		self.v = [0; 16];
		self.opcode = 0;
		self.i = 0;
		self.pc = 0;
		
		self.stack = [0;16]; 
		self.sp = 0;

		self.delay_timer=0;
		self.sound_timer=0;
		for i in 0 .. 80 {
			self.ram[i+0x50]=Cpu::fonts()[i];
		}
		self.keys = [false; 16];
		self.gfx = [[false;32];64];
		self.draw_flag=false;
	}

	pub fn new() -> Cpu {
		let mut c = Cpu {
			ram: [0; 4096],
			v: [0; 16],
			opcode: 0,
			i: 0,
			pc: 0,

			stack: [0;16],
			sp: 0,

			delay_timer: 0,
			sound_timer: 0,
			keys: [false; 16],
			gfx: [[false;32];64],
			draw_flag: false,
		};
		c.reset();
		c
	}
	
	pub fn get_gfx(&self) -> [[bool;32];64]{
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
					self.gfx = [[false; 32];64];
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
			7 => self.v[x as usize] = self.v[x as usize] + kk,
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
			0xD => {
				//TODO Graphics stuff
			},
			
			_ => println!("Unregonised opcode"),
		}
		
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
