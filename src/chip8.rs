use std;
use std::fs::File;
use std::io::prelude::*;

use cpu::Cpu;
use cpu::CpuState;

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
            // Each loop is ~ 1/120th of a second
            std::thread::sleep(std::time::Duration::from_millis(8));

            //Emulate one cycle
            self.cpu.step();

            self.cpu.decrease_timers();

            
        }
    }

    pub fn step(&mut self) -> CpuState {
        self.cpu.step()
    

        //TODO:
        //draw flag
        //input keys
    }

    pub fn decrease_timers(&mut self) {
        self.cpu.decrease_timers();
    }

    pub fn reset_keys(&mut self) {
        self.cpu.reset_keys();
    }
}