struct Stackpointer {
    bytes: u8,
}

impl Stackpointer {
    pub fn get(&self) -> usize {
        self.bytes as usize
    }

    pub fn increase(&mut self) {
        self.bytes += 1;
    }
}

struct Opcode {
    bytes: u16,
}

impl Opcode {
    pub fn get_full(&self) -> u16 {
        self.bytes
    }

    pub fn get_lower(&self) -> u8 {
        (self.bytes & 0x00FF) as u8
    }

    pub fn get_upper(&self) -> u8 {
        ((self.bytes & 0xFF00) >> 8) as u8
    }

    pub fn set_lower(&mut self, lower: u8) {
        self.bytes |= lower as u16;
    }

    pub fn set_upper(&mut self, upper: u8) {
        self.bytes |= (upper as u16) << 8;
    }

    pub fn get_instruction(&self) -> usize {
        ((self.bytes & 0xF000) >> 12) as usize
    }
    pub fn get_data(&self) -> u16 {
        self.bytes & 0x0FFF
    }
}

struct Chip8 {
    memory: Vec<u8>,
    v: Vec<u8>,
    stack: Vec<u16>,

    opcode: Opcode,

    i_reg: u16,
    pc: u16,
    sp: Stackpointer,
    /*
    unsigned char delay_timer;
    unsigned char sound_timer;*/
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let memory: Vec<u8> = vec![0; 4096]; //4096 bits of memory
        let v: Vec<u8> = vec![0; 16]; //CPU registers named V0 to VE, last register is the carry flag
        let stack: Vec<u16> = vec![0; 16]; //16 Stacklevels

        let opcode = Opcode { bytes: 0 };
        let i_reg: u16 = 0;
        let sp = Stackpointer { bytes: 0 };

        let pc: u16 = 0x200;

        Chip8 {
            memory,
            v,
            stack,
            opcode,
            i_reg,
            pc,
            sp,
        }
    }

    pub fn emulate_cycle(&mut self) {
        self.opcode.set_upper(self.memory[self.pc as usize]);
        println!("{:x}", self.opcode.get_upper());
        self.opcode.set_lower(self.memory[self.pc as usize + 1]);
        //println!("{:x}", self.opcode.get_full());
        //println!("{:x}", self.opcode.get_lower());

        let instruction = self.opcode.get_instruction();

        match self.opcode.get_instruction() {
            0x0 => {}
            0x1 => self.jump(),
            0x2 => self.call(),
            _ => {}
        }
    }

    fn jump(&mut self) {
        self.pc = self.opcode.get_data();
    }

    fn call(&mut self) {
        self.stack[self.sp.get()] = self.pc;
        self.sp.increase();
        self.pc = self.opcode.get_data();
    }
}

fn main() {
    let mut chip8: Chip8 = Chip8::new();
    //chip8.memory[0x200] = 1;
    //chip8.memory[0x201] = 2;
    //chip8.emulate_cycle();
    //println!("{}", chip8.opcode.get_full());
    //println!("{}", chip8.opcode.get_upper());
    //println!("{}", chip8.opcode.get_lower());
    //println!("{}", chip8.memory[1]);
    //println!("{}", chip8.memory[4095]);
    /*let mut quit = false;

    while !&quit {
        chip8.emulate_cycle();
    }*/
}

#[cfg(test)]
mod chip8_tests;
