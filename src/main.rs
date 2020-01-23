use rand::prelude::*;

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

    pub fn get_first_byte(&self) -> usize {
        (self.bytes & 0x000F) as usize
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
    sp: u8,

    i_reg: u16,

    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let memory: Vec<u8> = vec![0; 4096]; //4096 bits of memory
        let v: Vec<u8> = vec![0; 16]; //CPU registers named V0 to VE, last register is the carry flag
        let stack: Vec<u16> = vec![0; 16]; //16 Stacklevels

        let opcode = Opcode { bytes: 0 };
        let sp = 0;
        let pc = Programcounter { bytes: 0x200 };

        let i_reg = 0;

        let delay_timer = 0;
        let sound_timer = 0;

        Chip8 {
            memory,
            v,
            stack,
            opcode,
            i_reg,
            pc,
            sp,
            delay_timer,
            sound_timer,
        }
    }

    pub fn emulate_cycle(&mut self) {
        self.opcode.set(
            self.memory[self.pc.as_index()],
            self.memory[self.pc.as_index() + 1],
        );

        let nnn = self.opcode.get_3bytes();
        let nn = self.opcode.get_2bytes();
        let n = self.opcode.get_first_byte();
        let y = self.opcode.get_second_byte();
        let x = self.opcode.get_third_byte();

        match self.opcode.get_instruction() {
            0x0 => self.opcode0(nn),
            0x1 => self.jump(nnn),
            0x2 => self.call(nnn),
            0x3 => self.se(self.v[x], nn),
            0x4 => self.sne(self.v[x], nn),
            0x5 => self.se(self.v[x], self.v[y]),
            0x6 => self.ld(x, nn),
            0x7 => self.add(x, nn),
            0x8 => self.opcode8(n, x, y),
            0x9 => self.sne(self.v[x], self.v[y]),
            0xA => self.ldi(nnn),
            0xB => self.jump(nnn + self.v[0] as u16),
            0xC => self.ld(x, rand::random::<u8>() & nn),
            0xD => {} //TODO
            0xE => {} //TODO
            0xF => self.opcodef(x, y, n),
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
            self.sp -= 1;
            self.pc.set(self.stack[self.sp as usize]);
        }
        self.pc.next_instruction();
    }

    fn jump(&mut self, location: u16) {
        self.pc.set(location);
    }

    fn call(&mut self, location: u16) {
        self.stack[self.sp as usize] = self.pc.get();
        self.sp += 1;
        self.pc.set(location);
    }

    fn se(&mut self, x: u8, y: u8) {
        if x == y {
            self.pc.afternext_instruction();
        } else {
            self.pc.next_instruction();
        }
    }

    fn sne(&mut self, x: u8, y: u8) {
        if x != y {
            self.pc.afternext_instruction();
        } else {
            self.pc.next_instruction();
        }
    }

    fn ld(&mut self, register_number: usize, constant: u8) {
        self.v[register_number] = constant;
        self.pc.next_instruction();
    }

    fn add(&mut self, register_number: usize, constant: u8) {
        self.v[register_number] += constant;
        self.pc.next_instruction();
    }

    fn opcode8(&mut self, subcode: usize, x: usize, y: usize) {
        match subcode {
            0x0 => self.v[x] = self.v[y],
            0x1 => self.v[x] |= self.v[y],
            0x2 => self.v[x] &= self.v[y],
            0x3 => self.v[x] ^= self.v[y],
            0x4 => {}
            0x5 => {}
            0x6 => {
                self.v[15] = self.v[x] & 0b1;
                self.v[x] /= 2
            }
            0x7 => {}
            0xE => {}
            _ => {}
        }
    }

    fn ldi(&mut self, constant: u16) {
        self.i_reg = constant;
        self.pc.next_instruction();
    }

    fn opcodef(&mut self, x: usize, y: usize, n: usize) {
        match y {
            0x0 => {
                if n == 0x7 {
                    self.v[x] = self.delay_timer;
                    self.pc.next_instruction();
                } else if n == 0xA {
                    //TODO
                }
            }
            0x1 => {
                if n == 0x5 {
                    self.delay_timer = self.v[x];
                } else if n == 0x8 {
                    self.sound_timer = self.v[x];
                } else if n == 0xE {
                    let (ireg, overflow_bit) = self.i_reg.overflowing_add(self.v[x] as u16);
                    self.i_reg = ireg;
                    self.v[15] = overflow_bit as u8;
                }
            }
            0x2 => {}
            0x3 => {}
            0x5 => {
                for i in 0..x {
                    self.memory[self.i_reg as usize + i] = self.v[i];
                }
                self.pc.next_instruction();
            }
            0x6 => {
                for i in 0..x {
                    self.v[i] = self.memory[self.i_reg as usize + i];
                }
                self.pc.next_instruction();
            }
            _ => {}
        }
    }
}

fn main() {
    let mut chip8: Chip8 = Chip8::new();

    //chip8.memory[0x200] = 1;
    //chip8.memory[0x201] = 2;
    //chip8.emulate_cycle();
    println!("{}", rand::random::<u8>());
    //println!("{}", chip8.memory[1]);
    //println!("{}", chip8.memory[4095]);
    /*let mut quit = false;

    while !&quit {
        chip8.emulate_cycle();
    }*/
}

#[cfg(test)]
mod chip8_tests;
