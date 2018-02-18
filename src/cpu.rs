use std;
use ram::RAM;

pub struct Cpu {
    i:  u16,    //Index
    pc: u16,    //Program counter
    op: u16,    //Opcode

    V: [u8; 16], //V Registers

    stack: Vec<u16>, //Callstack
    sp: u16,    //Stack pointer

    pub ram: RAM,   //RAM

    gfx: [u8; 64 * 32]
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

            //TODO: 
            //Clear display
            //Clear regs
            //Load fontset
        }
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
                        println!("Instr: 00E0"); //CLS
                        //TODO: Clear screen.
                    },
                    0x000E => {
                        println!("Instr: 00EE"); //RET
                        self.pc = self.stack.pop().unwrap();
                        self.sp -= 1;
                    },
                    _ => {
                        println!("Unknown 0x0 instruction: {}! Exiting program...", instr);
                        std::process::exit(1);
                    }
                }
                
            },
            0x1000 => {
                println!("Instr: 1NNN");
                self.pc = nnn;
            },
            0x2000 => {
                println!("Instr: 2NNN");
                self.sp += 1;
                self.stack.push(self.pc);
                self.pc = nnn;
            },
            0x3000 => {
                println!("Instr: 3XKK");
                println!("VX: {}, KK: {}", self.V[x], kk);

                if self.V[x] == kk { self.pc += 2; }
            },
            0x6000 => {
                println!("Instr: 6XKK"); //LD Vx, byte
                self.V[x] = kk;
            },
            0x7000 => {
                println!("Instr: 7XKK"); //ADD Vx, byte
                self.V[x] = self.V[x].wrapping_add(kk);
            },
            0xA000 => {
                println!("Instr: ANNN");
                self.i = nnn;
            },
            0xD000 => {
                println!("Instr: DXYN"); //DRW Vx, Vy

                //TODO: Implement
                //TODO: DrawFlag
            }

            _ => {
                println!("Unrecognized Instruction {:#X}! Exiting program...", instr);
                std::process::exit(1);
            },
        }

        //TODO: Update Timers
    }
}