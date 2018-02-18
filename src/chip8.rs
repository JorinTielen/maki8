use std;
use std::fs::File;
use std::io::prelude::*;

use cpu::Cpu;

pub struct Chip8 {
    cpu: Cpu
}

impl Chip8 {
    pub fn new() -> Chip8 {
        //Constructor of Chip8
        Chip8 { 
            cpu: Cpu::new(),
        }
    }

    pub fn step(&mut self) {
        self.cpu.step();
    }

    pub fn load(&mut self, rom: File) {
        for (i, byte) in rom.bytes().enumerate() {
            let byte = match byte {
                Ok(byte) => byte,
                Err(msg) => {
                    println!("Error loading ROM: {}", msg);
                    std::process::exit(1);
                }
            };

            self.cpu.ram.write_byte(i + 512, byte);
        }
    }

    pub fn start(&mut self) {
        //for i in 0..140
        loop {
            //Emulate one cycle
            self.step();

            //TODO:
            //draw flag
            //input keys
        }
    }
}