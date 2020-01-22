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

    pub fn decrease(&mut self) {
        self.bytes -= 1;
    }
}

struct Opcode {
    bytes: u16,
}

impl Opcode {
    pub fn get(&self) -> u16 {
        self.bytes
    }

    pub fn set(&mut self, upper: u8, lower: u8) {
        self.bytes = (upper as u16) << 8 | lower as u16;
    }

    pub fn get_instruction(&self) -> usize {
        ((self.bytes & 0xF000) >> 12) as usize
    }

    pub fn get_3bytes(&self) -> u16 {
        self.bytes & 0x0FFF
    }

    pub fn get_2bytes(&self) -> u8 {
        (self.bytes & 0x00FF) as u8
    }

    pub fn get_third_byte(&self) -> usize {
        ((self.bytes & 0x0F00) >> 8) as usize
    }

    pub fn get_second_byte(&self) -> usize {
        ((self.bytes & 0x00F0) >> 4) as usize
    }
}

struct Programcounter {
    bytes: u16,
}

impl Programcounter {
    pub fn get(&self) -> u16 {
        self.bytes
    }

    pub fn as_index(&self) -> usize {
        self.bytes as usize
    }

    pub fn set(&mut self, pc: u16) {
        self.bytes = pc
    }

    pub fn next_instruction(&mut self) {
        self.bytes += 2;
    }
    pub fn afternext_instruction(&mut self) {
        self.bytes += 4;
    }
}

struct Chip8 {
    memory: Vec<u8>,
    v: Vec<u8>,
    stack: Vec<u16>,

    opcode: Opcode,
    pc: Programcounter,
    sp: Stackpointer,

    i_reg: u16,
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
        let sp = Stackpointer { bytes: 0 };
        let pc = Programcounter { bytes: 0x200 };

        let i_reg: u16 = 0;

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
        self.opcode.set(
            self.memory[self.pc.as_index()],
            self.memory[self.pc.as_index() + 1],
        );

        let xnnn = self.opcode.get_3bytes();
        let xxnn = self.opcode.get_2bytes();
        let xnxx = self.opcode.get_third_byte();
        let xxnx = self.opcode.get_second_byte();

        match self.opcode.get_instruction() {
            0x0 => self.opcode0(xxnn),
            0x1 => self.jump(xnnn),
            0x2 => self.call(xnnn),
            0x3 => self.se(self.v[xnxx], xxnn),
            0x5 => self.se(self.v[xnxx], self.v[xxnx]),
            _ => {}
        }
    }

    fn opcode0(&mut self, subcode: u8) {
        /*
        if (opcode == 0x00E0) { //CLS
            for (auto &i : gfx) {
                for (auto &&j : i) {
                    j = false;
                }
            }
        } else*/
        if subcode == 0xEE {
            self.sp.decrease();
            self.pc.set(self.stack[self.sp.get()]);
        }
        self.pc.next_instruction();
    }

    fn jump(&mut self, location: u16) {
        self.pc.set(location);
    }

    fn call(&mut self, location: u16) {
        self.stack[self.sp.get()] = self.pc.get();
        self.sp.increase();
        self.pc.set(location);
    }

    fn se(&mut self, a: u8, b: u8) {
        if a == b {
            self.pc.afternext_instruction();
        } else {
            self.pc.next_instruction();
        }
    }
}

fn main() {
    let mut chip8: Chip8 = Chip8::new();
    //chip8.memory[0x200] = 1;
    //chip8.memory[0x201] = 2;
    //chip8.emulate_cycle();
    //println!("{}", chip8.opcode.get());
    //println!("{}", chip8.memory[1]);
    //println!("{}", chip8.memory[4095]);
    /*let mut quit = false;

    while !&quit {
        chip8.emulate_cycle();
    }*/
}

#[cfg(test)]
mod chip8_tests;
