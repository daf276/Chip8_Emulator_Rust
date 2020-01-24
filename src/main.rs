extern crate clap;
extern crate rand;
extern crate sdl2;

use clap::{App, Arg};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::fs::File;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::{Duration, Instant};

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;

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
    gfx: Vec<Vec<u8>>,
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
        let gfx = vec![vec![0; 64]; 32];
        let mut memory = vec![0; 4096]; //4096 bits of memory
        let v = vec![0; 16]; //CPU registers named V0 to VE, last register is the carry flag
        let stack = vec![0; 16]; //16 Stacklevels

        let opcode = Opcode { bytes: 0 };
        let sp = 0;
        let pc = Programcounter { bytes: 0x200 };

        let i_reg = 0;

        let delay_timer = 0;
        let sound_timer = 0;

        memory = Chip8::load_hex_digits(memory);

        Chip8 {
            gfx,
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

    pub fn load_into_memory(&mut self, data: Vec<u8>) {
        self.memory[0x200..data.len() + 0x200].copy_from_slice(&data);
    }

    pub fn emulate_cycle(&mut self) {
        self.opcode.set(
            self.memory[self.pc.as_index()],
            self.memory[self.pc.as_index() + 1],
        );

        //TODO this can be done with one function and parameters
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
            0x4 => {
                let (reg, overflow_bit) = self.v[x].overflowing_add(self.v[y]);
                self.v[x] = reg;
                self.v[15] = overflow_bit as u8;
            }
            0x5 => {
                let (reg, overflow_bit) = self.v[x].overflowing_sub(self.v[y]);
                self.v[x] = reg;
                self.v[15] = overflow_bit as u8;
            }
            0x6 => {
                self.v[15] = self.v[x] & 0b1;
                self.v[x] /= 2
            }
            0x7 => {
                let (reg, overflow_bit) = self.v[y].overflowing_sub(self.v[x]);
                self.v[x] = reg;
                self.v[15] = overflow_bit as u8;
            }
            0xE => {
                self.v[15] = (self.v[x] & 0b10000000) >> 7;
                self.v[x] = self.v[x].wrapping_shl(1);
            }
            _ => {}
        }
        self.pc.next_instruction();
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

    fn load_hex_digits(mut memory: Vec<u8>) -> Vec<u8> {
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

        return memory;
    }
}

fn main() {
    let matches = App::new("Chip-8 Emulator")
        .version("0.1.0")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("A cool file"),
        )
        .get_matches();

    let myfile = matches.value_of("file");

    match myfile {
        Some(f) => {
            println!("The file passed is: {}", f);

            match File::open(f) {
                Ok(file) => emulate(load_program(file)),
                Err(_) => println!("File doesnt exist"),
            }
        }
        None => println!("No File passed"),
    }
}

fn emulate(mut chip: Chip8) {
    let sdl_context = sdl2::init().expect("Can't get SDLContext");
    let video_subsystem = sdl_context
        .video()
        .expect("Can't initialize video subsystem");
    let window = video_subsystem
        .window("Chip-8", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("Can't create window");
    let mut canvas = window.into_canvas().build().expect("Can't create canvas");
    let mut event_pump = sdl_context.event_pump().expect("Can't get event pump");

    let mut quit = false;

    while !&quit {
        let before_cycle = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    quit = true;
                }
                _ => {}
            }
        }

        chip.emulate_cycle();

        let time_to_wait =
            (1000000000u128 / 60u128).saturating_sub(before_cycle.elapsed().as_nanos()); //60Fps
        sleep(Duration::new(0, time_to_wait as u32));
    }
}

fn load_program(mut file: File) -> Chip8 {
    let mut chip8 = Chip8::new();

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    chip8.load_into_memory(buffer);

    return chip8;
}

#[cfg(test)]
mod chip8_tests;
