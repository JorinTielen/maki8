use std;
use ram::RAM;

pub struct Cpu {
    i:  u16,    //Index
    pc: u16,    //Program counter
    op: u16,    //Opcode

    V: [u16; 16], //V Registers

    stack: Vec<u16>, //Callstack
    sp: u16,    //Stack pointer

    pub ram: RAM,   //RAM
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

        //Maybe it would be logical to increment the pc here?
        //it would go wrong at 0x2000 though.

        match instr {
            0x0 => {
                println!("Instr: 00EE");
                self.pc = self.stack.pop().unwrap() + 2; //continue at the instruction after the one on the stack
            }
            0xA000 => {
                println!("Instr: ANNN");
                self.i = self.op & 0x0FFF;
                self.pc += 2;
            },
            0x2000 => {
                println!("Instr: 2NNN");
                self.sp += 1;
                self.stack.push(self.pc);
                self.pc = self.op & 0x0FFF;
            },
            0x6000 => {
                println!("Instr: 6XKK");
                self.V[((self.op << 8) & 0x0F) as usize] = self.op & 0x00FF;
                self.pc += 2;
            },
            _ => {
                println!("Unrecognized Instruction {:#X}! Exiting program...", instr);
                std::process::exit(1);
            },
        }

        //TODO: Update Timers
    }
}