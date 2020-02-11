use std::fs::File;
use std::io::prelude::*;
pub struct Chip8 {
    pub screen_scale: u32,
    pub gfx: [[bool; Chip8::SCREEN_WIDTH]; Chip8::SCREEN_HEIGHT],
    pub key_pressed: [bool; 16],
    memory: [u8; 4096],
    v: [u8; 16],
    stack: [u16; 16],
    opcode: u16,
    pc: u16,
    sp: u8,
    i_reg: u16,
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    pub const SCREEN_WIDTH: usize = 64;
    pub const SCREEN_HEIGHT: usize = 32;

    fn new() -> Chip8 {
        let gfx = [[false; Chip8::SCREEN_WIDTH]; Chip8::SCREEN_HEIGHT];
        let mut memory = [0; 4096]; //4096 bits of memory
        memory = Chip8::load_hex_digits(memory);

        Chip8 {
            gfx,
            key_pressed: [false; 16], //16 Keys
            memory,
            v: [0; 16],     //CPU registers named V0 to VE, last register is the carry flag
            stack: [0; 16], //16 Stacklevels
            opcode: 0,
            i_reg: 0,
            pc: 0x200, //Execution must start at 0x200
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            screen_scale: 1,
        }
    }

    pub fn create_chip(file: File, screen_scale: u32) -> Chip8 {
        let mut chip = Chip8::load_program(file);
        chip.screen_scale = screen_scale;
        chip
    }

    fn load_program(mut file: File) -> Chip8 {
        let mut chip8 = Chip8::new();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        chip8.load_into_memory(buffer);
        chip8
    }

    pub fn load_into_memory(&mut self, data: Vec<u8>) {
        self.memory[0x200..data.len() + 0x200].copy_from_slice(&data);
    }

    pub fn emulate_cycle(&mut self) {
        let opcode_upper_8bit = (self.memory[self.pc as usize] as u16) << 8;
        let opcode_lower_8bit = self.memory[self.pc as usize + 1] as u16;
        self.opcode = opcode_upper_8bit | opcode_lower_8bit;

        let instruction = (&self.opcode & 0xF000) >> 12;
        let nnn = self.opcode & 0x0FFF;
        let nn = (self.opcode & 0x00FF) as u8;
        let n = (self.opcode & 0x000F) as u8;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;
        let x = ((self.opcode & 0x0F00) >> 8) as usize;

        match instruction {
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
            0xD => self.display_sprite(x, y, n as usize),
            0xE => self.opcodee(x, nn),
            0xF => self.opcodef(x, y, n),
            _ => unreachable!(),
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            println!("Bzz"); //Todo actually make sound instead of console output
            self.sound_timer -= 1;
        }
    }

    fn opcode0(&mut self, subcode: u8) {
        if subcode == 0xE0 {
            self.gfx = [[false; 64]; 32];
        } else if subcode == 0xEE {
            self.sp -= 1;
            self.pc = self.stack[self.sp as usize];
        }
        self.pc += 2;
    }

    fn jump(&mut self, location: u16) {
        self.pc = location;
    }

    fn call(&mut self, location: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = location;
    }

    fn se(&mut self, x: u8, y: u8) {
        self.pc += if x == y { 4 } else { 2 };
    }

    fn sne(&mut self, x: u8, y: u8) {
        self.pc += if x != y { 4 } else { 2 };
    }

    fn ld(&mut self, register_number: usize, constant: u8) {
        self.v[register_number] = constant;
        self.pc += 2;
    }

    fn add(&mut self, register_number: usize, constant: u8) {
        self.v[register_number] = self.v[register_number].wrapping_add(constant);
        self.pc += 2;
    }

    fn opcode8(&mut self, subcode: u8, x: usize, y: usize) {
        match subcode {
            0x0 => self.v[x] = self.v[y],
            0x1 => self.v[x] |= self.v[y],
            0x2 => self.v[x] &= self.v[y],
            0x3 => self.v[x] ^= self.v[y],
            0x4 => {
                let (reg, overflow_bit) = self.v[x].overflowing_add(self.v[y]);
                self.v[x] = reg;
                self.v[15] = overflow_bit as u8;
            }
            0x5 => {
                let (reg, overflow_bit) = self.v[x].overflowing_sub(self.v[y]);
                self.v[x] = reg;
                self.v[15] = !overflow_bit as u8;
            }
            0x6 => {
                self.v[15] = self.v[x] & 0b1;
                self.v[x] >>= 1;
            }
            0x7 => {
                let (reg, overflow_bit) = self.v[y].overflowing_sub(self.v[x]);
                self.v[x] = reg;
                self.v[15] = !overflow_bit as u8;
            }
            0xE => {
                self.v[15] = (self.v[x] & 0b1000_0000) >> 7;
                self.v[x] <<= 1;
            }
            _ => {}
        }
        self.pc += 2;
    }

    fn ldi(&mut self, constant: u16) {
        self.i_reg = constant;
        self.pc += 2;
    }

    fn display_sprite(&mut self, x: usize, y: usize, n: usize) {
        self.v[15] = 0;
        //TODO bug: only one sid gets wrapped around correctly?
        //TODO this is unreadable, clean this shit up
        let bitmask = [128u8, 64, 32, 16, 8, 4, 2, 1];
        let bytes = self.memory[self.i_reg as usize..self.i_reg as usize + n].to_vec();
        let mut x_pos = [0usize; 8];
        let mut y_pos = vec![0usize; n];

        //Modulo with screen size because sprites get wrapped around the display
        for i in 0..8 {
            x_pos[i] = (self.v[x] as usize + i) % Chip8::SCREEN_WIDTH;
        }
        for i in 0..n {
            y_pos[i] = (self.v[y] as usize + i) % Chip8::SCREEN_HEIGHT;
        }

        for (&y, &byte) in y_pos.iter().zip(bytes.iter()) {
            for (&x, &bit) in x_pos.iter().zip(bitmask.iter()) {
                let pixel_set = (byte & bit) != 0;

                if pixel_set && self.gfx[y][x] {
                    self.v[15] = 1;
                }

                self.gfx[y][x] ^= pixel_set;
            }
        }

        self.pc += 2;
    }

    fn opcodee(&mut self, x: usize, nn: u8) {
        let reg = self.v[x] as usize;
        if nn == 0x9E {
            self.pc += if self.key_pressed[reg] { 4 } else { 2 };
        } else if nn == 0xA1 {
            self.pc += if self.key_pressed[reg] { 2 } else { 4 };
        }
    }

    fn opcodef(&mut self, x: usize, y: usize, n: u8) {
        match y {
            0x0 => {
                if n == 0x7 {
                    self.v[x] = self.delay_timer;
                    self.pc += 2;
                } else if n == 0xA {
                    if let Some(y) = self.key_pressed.iter().find(|&&y| y) {
                        self.v[x] = *y as u8;
                        self.pc += 2;
                    }
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
                self.pc += 2;
            }
            0x2 => {
                //Set i to location of sprite in vx
                //This works because sprites are 5bytes long and begin at memory address 0x0
                self.i_reg = self.v[x] as u16 * 5;
                self.pc += 2;
            }
            0x3 => {
                self.memory[self.i_reg as usize] = self.v[x] / 100;
                self.memory[self.i_reg as usize + 1] = (self.v[x] / 10) % 10;
                self.memory[self.i_reg as usize + 2] = (self.v[x] % 100) % 10;
                self.pc += 2;
            }
            0x5 => {
                for i in 0..x {
                    self.memory[self.i_reg as usize + i] = self.v[i];
                }
                self.pc += 2;
            }
            0x6 => {
                for i in 0..x {
                    self.v[i] = self.memory[self.i_reg as usize + i];
                }
                self.pc += 2;
            }
            _ => {}
        }
    }

    fn load_hex_digits(mut memory: [u8; 4096]) -> [u8; 4096] {
        //Zero
        memory[0] = 0xF0;
        memory[1] = 0x90;
        memory[2] = 0x90;
        memory[3] = 0x90;
        memory[4] = 0xF0;
        //One
        memory[5] = 0x20;
        memory[6] = 0x60;
        memory[7] = 0x20;
        memory[8] = 0x20;
        memory[9] = 0x70;
        //Two
        memory[10] = 0xF0;
        memory[11] = 0x10;
        memory[12] = 0xF0;
        memory[13] = 0x80;
        memory[14] = 0xF0;
        //Three
        memory[15] = 0xF0;
        memory[16] = 0x10;
        memory[17] = 0xF0;
        memory[18] = 0x10;
        memory[19] = 0xF0;
        //Four
        memory[20] = 0x90;
        memory[21] = 0x90;
        memory[22] = 0xF0;
        memory[23] = 0x10;
        memory[24] = 0x10;
        //Five
        memory[25] = 0xF0;
        memory[26] = 0x80;
        memory[27] = 0xF0;
        memory[28] = 0x10;
        memory[29] = 0xF0;
        //Six
        memory[30] = 0xF0;
        memory[31] = 0x80;
        memory[32] = 0xF0;
        memory[33] = 0x90;
        memory[34] = 0xF0;
        //Seven
        memory[35] = 0xF0;
        memory[36] = 0x10;
        memory[37] = 0x20;
        memory[38] = 0x40;
        memory[39] = 0x40;
        //Eight
        memory[40] = 0xF0;
        memory[41] = 0x90;
        memory[42] = 0xF0;
        memory[43] = 0x90;
        memory[44] = 0xF0;
        //Nine
        memory[45] = 0xF0;
        memory[46] = 0x90;
        memory[47] = 0xF0;
        memory[48] = 0x10;
        memory[49] = 0xF0;
        //A
        memory[50] = 0xF0;
        memory[51] = 0x90;
        memory[52] = 0xF0;
        memory[53] = 0x90;
        memory[54] = 0x90;
        //B
        memory[55] = 0xE0;
        memory[56] = 0x90;
        memory[57] = 0xE0;
        memory[58] = 0x90;
        memory[59] = 0xE0;
        //C
        memory[60] = 0xF0;
        memory[61] = 0x80;
        memory[62] = 0x80;
        memory[63] = 0x80;
        memory[64] = 0xF0;
        //D
        memory[65] = 0xE0;
        memory[66] = 0x90;
        memory[67] = 0x90;
        memory[68] = 0x90;
        memory[69] = 0xE0;
        //E
        memory[70] = 0xF0;
        memory[71] = 0x80;
        memory[72] = 0xF0;
        memory[73] = 0x80;
        memory[74] = 0xF0;
        //F
        memory[75] = 0xF0;
        memory[76] = 0x80;
        memory[77] = 0xF0;
        memory[78] = 0x80;
        memory[79] = 0x80;

        memory
    }
}

#[cfg(test)]
mod tests;
