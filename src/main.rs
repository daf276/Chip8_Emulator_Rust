extern crate clap;
extern crate rand;
extern crate sdl2;

mod chip8;

use clap::{App, Arg};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::Sdl;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::{Duration, Instant};

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const PITCH: usize = SCREEN_WIDTH * 4;
const SCALE: u32 = 16;

fn main() {
    let matches = App::new("Chip-8 Emulator")
        .version("0.2.0")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("A cool file"),
        )
        .get_matches();

    match matches.value_of("file") {
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

fn emulate(mut chip: chip8::Chip8) {
    let (mut event_pump, mut canvas) = init_sdl();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_static(
            PixelFormatEnum::ABGR8888,
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )
        .expect("Can't create texture");

    let mut quit = false;

    while !&quit {
        let before_cycle = Instant::now();

        quit = quit_event_activated(&mut event_pump);

        let keys: HashSet<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        chip.key_pressed = map_keys(keys);

        chip.emulate_cycle();

        let pixel_data = update_gfx(&chip.gfx);
        texture.update(None, &pixel_data[..], PITCH).unwrap();

        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        //let time_to_wait = 16666666u128.saturating_sub(before_cycle.elapsed().as_nanos()); //60Fps
        //sleep(Duration::new(0, time_to_wait as u32));
    }
}

fn init_sdl() -> (EventPump, Canvas<Window>) {
    let sdl_context = sdl2::init().expect("Can't get SDLContext");

    let video_subsystem = sdl_context
        .video()
        .expect("Can't initialize video subsystem");
    let window = video_subsystem
        .window(
            "Chip-8",
            SCALE * SCREEN_WIDTH as u32,
            SCALE * SCREEN_HEIGHT as u32,
        )
        .position_centered()
        .build()
        .expect("Can't create window");
    let mut canvas = window.into_canvas().build().expect("Can't create canvas");
    canvas.set_scale(SCALE as f32, SCALE as f32).unwrap();

    let mut event_pump = sdl_context.event_pump().expect("Can't get event pump");

    return (event_pump, canvas);
}

fn map_keys(keys: HashSet<Keycode>) -> Vec<bool> {
    let mut key_pressed = vec![false; 16];

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

    return key_pressed;
}

fn quit_event_activated(event_pump: &mut EventPump) -> bool {
    for event in event_pump.poll_iter() {
        if let Event::Quit { .. } = event {
            return true;
        }
    }

    return false;
}

fn load_program(mut file: File) -> chip8::Chip8 {
    let mut chip8 = chip8::Chip8::new();

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    chip8.load_into_memory(buffer);

    return chip8;
}

fn update_gfx(emulator_gfx: &Vec<Vec<bool>>) -> Vec<u8> {
    let mut gfx = vec![0; SCREEN_HEIGHT * SCREEN_WIDTH];

    for i in 0..SCREEN_HEIGHT {
        for j in 0..SCREEN_WIDTH {
            let pixel = emulator_gfx[i][j] as u32 * 0xFFFFFFFF;
            gfx[i * SCREEN_WIDTH + j] = pixel;
        }
    }

    return split_gfx_into_color_components(gfx);
}

fn split_gfx_into_color_components(gfx: Vec<u32>) -> Vec<u8> {
    unsafe {
        let (_, ret, _) = gfx.align_to::<u8>();
        return ret.to_vec();
    }
}
