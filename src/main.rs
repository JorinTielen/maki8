use std::env;
use std::fs::File;

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

    //Start emulating!
    loop {
        //Emulate one cycle
        chip8.step();

        //TODO:
        //draw flag
        //input keys
    }
}
