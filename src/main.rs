extern crate clap;
extern crate rand;
extern crate sdl2;

mod chip8;

use clap::{App, Arg};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const SCREEN_WIDTH: usize = 64; //Todo maybe make these members of the chip8 struct?
const SCREEN_HEIGHT: usize = 32;
const PITCH: usize = SCREEN_WIDTH * 4;

fn main() {
    let matches = App::new("Chip-8 Emulator")
        .version("0.3.0")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("File to be emulated"),
        )
        .arg(
            Arg::with_name("scale")
                .short("s")
                .long("scale")
                .takes_value(true)
                .help("1x is 64*32"),
        )
        .get_matches();

    let screen_scale: u32 = match matches.value_of("scale") {
        Some(x) => match x.parse() {
            Ok(y) => y,
            Err(_) => 1,
        },
        None => 1,
    };

    match matches.value_of("file") {
        Some(f) => match File::open(f) {
            Ok(file) => emulate(load_program(file), screen_scale),
            Err(_) => println!("File doesnt exist"),
        },
        None => println!("No File passed"),
    }
}

fn load_program(mut file: File) -> chip8::Chip8 {
    let mut chip8 = chip8::Chip8::new();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    chip8.load_into_memory(buffer);
    return chip8;
}

fn emulate(mut chip: chip8::Chip8, screen_scale: u32) {
    let (mut event_pump, mut canvas) = init_sdl(screen_scale);
    let texture_creator = canvas.texture_creator(); //Todo find a better solution for this
    let mut texture = texture_creator
        .create_texture_static(
            PixelFormatEnum::ABGR8888,
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )
        .expect("Can't create texture");

    let mutex = Arc::new(Mutex::new(vec![vec![false; 64]; 32]));
    let thread_mutex = mutex.clone();

    thread::spawn(move || loop {
        let before_cycle = Instant::now();
        //chip.key_pressed = map_keys(&mut event_pump);
        chip.emulate_cycle();

        *thread_mutex.lock().unwrap() = chip.gfx.clone();

        let time_to_wait = 50000u128.saturating_sub(before_cycle.elapsed().as_nanos());
        thread::sleep(Duration::new(0, time_to_wait as u32));
        //println!("{}", time_to_wait);
    });

    let mut quit = false;

    while !&quit {
        let before_cycle = Instant::now();

        quit = quit_event_activated(&mut event_pump);
        //chip.key_pressed = map_keys(&mut event_pump);
        //chip.emulate_cycle();

        let pixel_data = update_gfx(&mutex.lock().unwrap().clone());
        texture.update(None, &pixel_data[..], PITCH).unwrap();

        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        let time_to_wait = 16_666_666_u128.saturating_sub(before_cycle.elapsed().as_nanos()); //60Fps
                                                                                              //println!("{}", time_to_wait);
        thread::sleep(Duration::new(0, time_to_wait as u32));
    }
}

fn init_sdl(screen_scale: u32) -> (EventPump, Canvas<Window>) {
    let sdl_context = sdl2::init().expect("Can't get SDLContext");
    let video_subsystem = sdl_context
        .video()
        .expect("Can't initialize video subsystem");
    let window = video_subsystem
        .window(
            "Chip-8",
            screen_scale * SCREEN_WIDTH as u32,
            screen_scale * SCREEN_HEIGHT as u32,
        )
        .position_centered()
        .build()
        .expect("Can't create window");
    let mut canvas = window.into_canvas().build().expect("Can't create canvas");
    canvas
        .set_scale(screen_scale as f32, screen_scale as f32)
        .unwrap();

    let event_pump = sdl_context.event_pump().expect("Can't get event pump");

    (event_pump, canvas)
}

fn quit_event_activated(event_pump: &mut EventPump) -> bool {
    for event in event_pump.poll_iter() {
        if let Event::Quit { .. } = event {
            return true;
        }
    }
    false
}

fn map_keys(event_pump: &mut EventPump) -> Vec<bool> {
    let keys: HashSet<Keycode> = event_pump
        .keyboard_state()
        .pressed_scancodes()
        .filter_map(Keycode::from_scancode)
        .collect();

    let mut key_pressed = vec![false; 16];

    //Todo find a better solution for this and put the keys in a config file
    for key in keys {
        match key {
            Keycode::Num1 => key_pressed[1] = true,
            Keycode::Num2 => key_pressed[2] = true,
            Keycode::Num3 => key_pressed[3] = true,
            Keycode::Num4 => key_pressed[0xC] = true,
            Keycode::Q => key_pressed[4] = true,
            Keycode::W => key_pressed[5] = true,
            Keycode::E => key_pressed[6] = true,
            Keycode::R => key_pressed[0xD] = true,
            Keycode::A => key_pressed[7] = true,
            Keycode::S => key_pressed[8] = true,
            Keycode::D => key_pressed[9] = true,
            Keycode::F => key_pressed[0xE] = true,
            Keycode::Y => key_pressed[0xA] = true,
            Keycode::X => key_pressed[0] = true,
            Keycode::C => key_pressed[0xB] = true,
            Keycode::V => key_pressed[0xF] = true,
            _ => {}
        }
    }

    key_pressed
}

fn update_gfx(emulator_gfx: &[Vec<bool>]) -> Vec<u8> {
    let mut gfx = vec![0; SCREEN_HEIGHT * SCREEN_WIDTH];

    for i in 0..SCREEN_HEIGHT {
        for j in 0..SCREEN_WIDTH {
            gfx[i * SCREEN_WIDTH + j] = emulator_gfx[i][j] as u32 * 0xFFFF_FFFF;
        }
    }

    split_gfx_into_color_components(gfx)
}

fn split_gfx_into_color_components(gfx: Vec<u32>) -> Vec<u8> {
    unsafe {
        let (_, ret, _) = gfx.align_to::<u8>();
        ret.to_vec()
    }
}
