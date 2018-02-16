use std;
use ram::RAM;

pub struct Cpu {
    i:  u16,    //Index
    pc: u16,    //Program counter
    op: u16,    //Opcode

    stack: [u16; 16], //Callstack
    sp: u16,    //Stack pointer

    pub ram: RAM,   //RAM
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            i: 0,
            pc: 0x200,
            op: 0,

            stack: [0; 16],
            sp: 0,

            ram: RAM::new(),

            //TODO: 
            //Clear display
            //Clear regs
            //Load fontset
        }
    }

    pub fn step(&mut self) {
        let ram = &self.ram;

        //Fetch opcode
        /*
        let first_byte = ram.read_byte(pc as usize);
        println!("1st byte: {:b}", first_byte);

        let second_byte = ram.read_byte((pc as usize) + 1);
        println!("2nd byte: {:b}", second_byte);
        */

        self.op = ram.read_u16(self.pc);
        println!("Opcode: {:#X}", self.op);

        let instr = self.op & 0xF000;

        match instr {
            0xA000 => {
                println!("Instr: ANNN");
                self.i = self.op & 0x0FFF;
                self.pc += 2;
            }
            _ => {
                println!("Unrecognized Instruction! Exiting program...");
                std::process::exit(1);
            }
        }
    }
}