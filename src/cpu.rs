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
		let kk  = (opcode & 0xFF);
		//Match top four bits
		match (opcode & 0xF000)>>12  {
			0 => match(opcode) {
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
				println!("3!");		
			},
			_ => println!("Sys ADDR, do nothing"),
		}
		if self.delay_timer > 0{
			self.delay_timer -= 1;
		}
		if self.sound_timer > 0 {
			//Do audio
			println!("PRETEND THERE WAS SOUND");
			self.sound_timer -= 1;
		}

	}
	fn get_opcode(&self) -> u16{
		(self.ram[self.pc as usize] as u16) << 8 | (self.ram[(self.pc as usize) + 1] as u16)
	}	
}
