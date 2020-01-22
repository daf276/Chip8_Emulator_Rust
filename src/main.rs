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

    pub fn get_data(&self) -> u16 {
        self.bytes & 0x0FFF
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

        let i_reg: u16 = 0;
        let pc = Programcounter { bytes: 0x200 };

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

        match self.opcode.get_instruction() {
            0x0 => self.opcode0(),
            0x1 => self.jump(),
            0x2 => self.call(),
            _ => {}
        }
    }

    fn opcode0(&mut self) {
        /*
        if (opcode == 0x00E0) { //CLS
            for (auto &i : gfx) {
                for (auto &&j : i) {
                    j = false;
                }
            }
        } else*/
        if self.opcode.get() == 0x00EE {
            //RET
            self.sp.decrease();
            self.pc.set(self.stack[self.sp.get()]);
        }
        self.pc.next_instruction();
    }

    fn jump(&mut self) {
        self.pc.set(self.opcode.get_data());
    }

    fn call(&mut self) {
        self.stack[self.sp.get()] = self.pc.get();
        self.sp.increase();
        self.pc.set(self.opcode.get_data());
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
