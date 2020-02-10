extern crate clap;
extern crate rand;
extern crate sdl2;

mod chip8;

use crate::chip8::Chip8;
use clap::{App, Arg};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::collections::HashSet;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const PITCH: usize = (Chip8::SCREEN_WIDTH * 4) as usize;

fn main() {
    let matches = App::new("Chip-8 Emulator")
        .version("0.4.0")
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
            Ok(file) => emulate(Chip8::create_chip(file, screen_scale)),
            Err(_) => println!("File doesnt exist"),
        },
        None => println!("No File passed"),
    }
}

fn emulate(chip: Chip8) {
    //Todo check if better solution for this exists
    let (mut event_pump, mut canvas) =
        init_sdl(Chip8::SCREEN_WIDTH, Chip8::SCREEN_HEIGHT, chip.screen_scale);
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_static(
            PixelFormatEnum::ABGR8888,
            Chip8::SCREEN_WIDTH,
            Chip8::SCREEN_HEIGHT,
        )
        .unwrap();

    let gfx = Arc::new(Mutex::new(vec![vec![false; 64]; 32]));
    let keys = Arc::new(Mutex::new(vec![false; 16]));

    start_logic_thread(chip, keys.clone(), gfx.clone());

    while !quit_event_activated(&mut event_pump) {
        //TODO try to make a render function out of this if texture lets me
        let before_cycle = Instant::now();

        *keys.lock().unwrap() = map_keys(&mut event_pump);

        let pixel_data = update_gfx(
            &gfx.lock().unwrap().clone(),
            Chip8::SCREEN_WIDTH as usize,
            Chip8::SCREEN_HEIGHT as usize,
        );

        texture.update(None, &pixel_data[..], PITCH).unwrap();

        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        let time_to_wait = 16_666_666_u128.saturating_sub(before_cycle.elapsed().as_nanos()); //60Fps
                                                                                              //println!("{}", time_to_wait);
        thread::sleep(Duration::new(0, time_to_wait as u32));
    }
}

fn start_logic_thread(
    mut chip: Chip8,
    keys: Arc<Mutex<Vec<bool>>>,
    gfx: Arc<Mutex<Vec<Vec<bool>>>>,
) {
    thread::spawn(move || loop {
        let before_cycle = Instant::now();

        chip.key_pressed = keys.lock().unwrap().clone();
        chip.emulate_cycle();
        *gfx.lock().unwrap() = chip.gfx.clone();

        let time_to_wait = 50000u128.saturating_sub(before_cycle.elapsed().as_nanos()); //20k ips
        thread::sleep(Duration::new(0, time_to_wait as u32));
        //println!("{}", time_to_wait);
    });
}

fn init_sdl(
    screen_width: u32,
    screen_height: u32,
    screen_scale: u32,
) -> (EventPump, Canvas<Window>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(
            "Chip-8",
            screen_scale * screen_width,
            screen_scale * screen_height,
        )
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas
        .set_scale(screen_scale as f32, screen_scale as f32)
        .unwrap();

    let event_pump = sdl_context.event_pump().unwrap();

    (event_pump, canvas)
}

fn quit_event_activated(event_pump: &mut EventPump) -> bool {
    event_pump.poll_iter().any(|x| {
        if let Event::Quit { .. } = x {
            true
        } else {
            false
        }
    })
}

fn map_keys(event_pump: &mut EventPump) -> Vec<bool> {
    //Todo find a better solution for this and put the keys in a config file
    let keys: HashSet<Keycode> = event_pump
        .keyboard_state()
        .pressed_scancodes()
        .filter_map(Keycode::from_scancode)
        .collect();

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

    key_pressed
}

fn update_gfx(emulator_gfx: &[Vec<bool>], screen_width: usize, screen_height: usize) -> Vec<u8> {
    let mut gfx = vec![0; screen_width * screen_height];

    for i in 0..screen_height {
        for j in 0..screen_width {
            gfx[i * screen_width + j] = emulator_gfx[i][j] as u32 * 0xFFFF_FFFF;
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
