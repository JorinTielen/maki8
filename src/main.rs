extern crate sdl2;

use std::env;
use std::fs::File;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod chip8;
mod cpu;
mod ram;

use chip8::Chip8;

fn main() {
    //read the ROM Path from the args
    let rom_path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Please provide a rom file. Exiting program...");
            std::process::exit(1);
        }
    };

    //Get the ROM File
    let rom = match File::open(&rom_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Error opening file at {}. Exiting program...", rom_path);
            std::process::exit(1);
        }
    };

    //setup the chip8
    let mut chip8 = Chip8::new();
    chip8.load(rom);

    //Setup SDL2 Context
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    //Setup Window
    let window = video_subsystem.window("maki8", 800, 600).resizable().build().unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let mut frame = 0;
    let mut event_pump = sdl_context.event_pump().unwrap();

    //Main SDL loop
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        //run the cpu faster than the refresh rate.
        for _ in 1..12 {
            chip8.step();
        }

        chip8.decrease_timers();

        chip8.reset_keys();
        
        frame += 1;
        canvas.clear();
        canvas.present();
    }
}

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}
