pub struct RAM {
    mem: [u8; 4096],
}

impl RAM {
    pub fn new() -> RAM {
        RAM { mem: [0; 4096] }
    }

    //Writes byte to mem at index i
    pub fn write_byte(&mut self, i: usize, byte: u8) {
        self.mem[i] = byte;
    }

    //Reads byte from mem at index i
    pub fn read_byte(&self, i: usize) -> u8 {
        self.mem[i]
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        let i = addr as usize;

        (self.mem[i] as u16) << 8 | (self.mem[i + 1] as u16)
    }
}