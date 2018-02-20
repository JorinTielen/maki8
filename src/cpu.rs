extern crate rand;

use std;
use ram::RAM;
use self::rand::Rng;

pub struct Cpu {
    i:  u16,    //Index
    pc: u16,    //Program counter
    op: u16,    //Opcode

    V: [u8; 16], //V Registers

    stack: Vec<u16>, //Callstack
    sp: u16, //Stack pointer

    pub ram: RAM, //RAM

    gfx: [u8; 64 * 32], //Display buffer

    key: [bool; 16], //Keypad - false if not pressed, true if pressed

    dt: u8, //Delay Timer
    st: u8, //Sound Timer
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            i: 0,
            pc: 0x200,
            op: 0,

            V: [0; 16],

            stack: Vec::new() , //This should be limited to 16 elements.
            sp: 0,

            ram: RAM::new(),

            gfx: [0; 64 * 32],

            key: [false; 16],

            dt: 0, //Delay Timer
            st: 0, //Sound Timer

            //TODO: 
            //Clear display
            //Clear regs
            //Load fontset
        }
    }

    pub fn decrease_timers(&mut self) {
        if self.dt > 0 { self.dt -= 1; }
        if self.st > 0 { self.st -= 1; }
    }

    pub fn step(&mut self) {
        let ram = &self.ram;

        //Fetch opcode
        self.op = ram.read_u16(self.pc);
        println!("Opcode: {:#X}", self.op);

        //Load instruction
        let instr = self.op & 0xF000;
        self.pc += 2;

        //Load common variables
        let x = ((self.op & 0x0F00) >> 8) as usize;
        let y = ((self.op & 0x00F0) >> 4) as usize;
        let nnn = self.op & 0x0FFF;
        let kk: u8 = (self.op & 0x00FF) as u8;

        //Execute instruction
        match instr {
            0x0 => {
                match self.op & 0x000F {
                    0x0000 => {
                        //00E0 - CLS
                        //Clear the Display.
                        println!("Instr: 00E0"); 
                        //TODO: Clear screen.
                    },
                    0x000E => {
                        //00EE - RET
                        //Return from a subroutine.
                        println!("Instr: 00EE");
                        self.pc = self.stack.pop().unwrap();
                        self.sp -= 1;
                    },
                    _ => {
                        //Should never happen, because only 2 0x0 instructions exist.
                        println!("Unknown 0x0 instruction: {}! Exiting program...", instr);
                        std::process::exit(1);
                    }
                }
                
            },
            0x1000 => {
                //1nnn - JP addr
                //Jump to location nnn.
                println!("Instr: 1nnn");
                self.pc = nnn;
            },
            0x2000 => {
                //2nnn - CALL addr
                //Call subroutine at nnn.
                println!("Instr: 2nnn");
                self.sp += 1;
                self.stack.push(self.pc);
                self.pc = nnn;
            },
            0x3000 => {
                //3xkk - SE Vx, kk
                //Skip next instruction if Vx == kk.
                println!("Instr: 3xkk");

                if self.V[x] == kk { self.pc += 2; }
            },
            0x4000 => {
                //4xkk - SNE Vx, kk
                //Skip next instruction if Vx != kk.
                println!("Instr: 4xkk");

                if self.V[x] != kk { self.pc += 2; }
            },
            0x6000 => {
                //6xkk - LD Vx, kk
                //Set Vx = kk.
                println!("Instr: 6xkk");
                self.V[x] = kk;
            },
            0x7000 => {
                //7xkk - ADD Vx, kk
                //Adds value kk to value in register Vx.
                println!("Instr: 7xkk");
                self.V[x] = self.V[x].wrapping_add(kk);
            },
            0xA000 => {
                //Annn - LD I, addr
                //The value of register I is set to nnn.
                println!("Instr: Annn");
                self.i = nnn;
            },
            0xC000 => {
                //Cxkk - RND Vx, kk
                //Adds a random value to kk, and stored in Vx.
                println!("Instr: Cxkk");

                let mut rng = rand::thread_rng();
                let mut value = rng.gen::<u8>();

                value.wrapping_add(kk); //Should this overflow? probably.

                self.V[x] = value;
            },
            0xD000 => {
                //Dxyn - DRW Vx, Vy, n
                //Disply n-byte sprite starting at memory location I at (Vx, Vy),
                //set VF = collision. (http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#Dxyn)
                println!("Instr: Dxyn");

                //TODO: Implement
                //TODO: DrawFlag
            },
            0xE000 => {
                let vx = self.V[x] as usize;
                match kk {
                    0x009E => {
                        //Ex9E - SKP Vx
                        //Skips the next instruction if the key with the value of Vx is pressed.
                        if vx < self.key.len() {
                            if self.key[vx] == true { self.pc += 2; }
                        }
                    },
                    0x00A1 => {
                        //ExA1 - SKNP Vx
                        //Skips the next instruction if the key with the value of Vx is not pressed.
                        if vx < self.key.len() {
                            if self.key[vx] == false { self.pc += 2; }
                        }
                    },
                    _ => {
                        //Shouldn't happen because there are only 2 known 0xE instructions.
                        println!("Unknown 0xE instruction: {}! Exiting program...", instr);
                        std::process::exit(1);
                    }
                }
            }
            0xF000 => {
                //Fx07 - LD Vx, DT
                //Set Vx = Delay Timer value.
                println!("Instr: Fx07");
                self.V[x] = self.dt;
            },

            _ => {
                println!("Unrecognized Instruction {:#X}! Exiting program...", instr);
                std::process::exit(1);
            },
        }

        //TODO: Update Timers
    }
}