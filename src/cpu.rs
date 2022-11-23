use crate::font;

pub struct cpu {
    pub ram: [u8; 4096],
    pub display: [[bool; 64]; 32],
    pub pc: u16,
    pub i: u16,
    pub stack: [u16; 16],
    pub sc: u16,
    pub delaytime: u8,
    pub soundtime: u8,
    pub v: [u8; 16],
}

impl cpu {
    pub fn new() -> cpu {
        cpu {
            ram: [0; 4096],
            display: [[false; 64]; 32],
            pc: 0x200,
            i: 0,
            stack: [0; 16],
            sc: 0,
            delaytime: 0,
            soundtime: 0,
            v: [0; 16],
        }
    }

    pub fn inject_code(&mut self, code: &[u8]) {
        let mut pindex: usize = 0;
        for ibm_byte in code {
            self.ram[0x200 + pindex] = ibm_byte.clone();
            pindex = pindex + 1;
        }
    }

    pub fn inject_font(&mut self, pfont: [u8; 80]) {
        let mut findex: usize = 0;
        for font_byte in pfont.iter() {
            self.ram[0x050 + findex] = font_byte.clone();
            findex = findex + 1;
        }
    }

    pub fn init(&mut self, code: &[u8]) {
        self.inject_font(font::FONT_MEM);
        self.inject_code(code);
    }

    pub fn step(&mut self) {
        //Fetch
        let opcode =
            (self.ram[self.pc as usize] as u16) << 8 | (self.ram[self.pc as usize + 1] as u16);
        let halfbyte = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );
        let nnn = (opcode & 0x0FFF) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let x = halfbyte.1 as usize;
        let y = halfbyte.2 as usize;
        let n = halfbyte.3 as usize;

        //Incriment PC
        self.pc = self.pc + 2;

        //Decode then Execute
        match halfbyte {
            (0x00, 0x00, 0x0e, 0x00) => {
                //Clear Screen
                self.display = [[false; 64]; 32];
            }
            (0x00,0x00,0x0e,0x0e) =>{
                self.pc = self.stack[self.sc as usize];
                self.sc -= 1;
            }
            (0x01,_,_,_) => { //Jump to NNN
                self.pc = nnn as u16;
            }
            (0x02,_,_,_) => {
                self.stack[self.sc as usize] = self.pc +2;
                self.sc += 1;
            }
            (0x03,_,_,_) =>{ //Skip If VX=KK
                if self.v[x] == kk{
                    self.pc += 2;
                }
            }
            (0x04,_,_,_) =>{ //Skip If VX=KK
                if self.v[x] != kk{
                    self.pc += 2;
                }
            }
            (0x05,_,_,_) =>{ //Skip If VX=KK
                if self.v[x] == self.v[y]{
                    self.pc += 2;
                }
            }
            (0x06, _, _, _) => {
                //Set Register VX
                self.v[x] = kk;
            }
            (0x07, _, _, _) => {
                //Add With No Carry VX
                self.v[x] = self.v[x].wrapping_add(kk);
            }
            (0x08,_,_,0x00) =>{
                self.v[x] = self.v[y]
            }
            (0x08,_,_,0x01) =>{
                self.v[x] |= self.v[y]
            }
            (0x08,_,_,0x02) =>{
                self.v[x] &= self.v[y]
            }
            (0x08,_,_,0x03) =>{
                self.v[x] ^= self.v[y]
            }
            (0x08,_,_,0x04) =>{
            
                let vx = self.v[x] as u16;
                let vy = self.v[y] as u16;
                let result = vx + vy;
                self.v[x] = result as u8;
                self.v[0x0f] = if result > 0xFF { 1 } else { 0 };

            }
            (0x08,_,_,0x05) =>{
                self.v[0x0f] = if self.v[x] > self.v[y] { 1 } else { 0 };
                self.v[x] = self.v[x].wrapping_sub(self.v[y]);
            }
            (0x08,_,_,0x06) =>{
                self.v[0x0f] = self.v[x] & 1;
                self.v[x] >>= 1;
            }
            (0x08,_,_,0x07) =>{
                self.v[0x0f] = if self.v[y] > self.v[x] { 1 } else { 0 };
                self.v[x] = self.v[y].wrapping_sub(self.v[x]);
            }
            (0x08,_,_,0x0e) =>{

                self.v[0x0f] = (self.v[x] & 0b10000000) >> 7;
                self.v[x] <<= 1;

            }
            (0x09,_,_,0x00) =>{
                if self.v[x] != self.v[y]{
                    self.pc += 2;
                }
            }
            (0x0a, _, _, _) => {
                // Set Index Register I
                self.i = nnn as u16;
            }
            (0x0b, _, _, _) => {
                // Set Index Register I
                self.pc = (nnn + self.v[0] as usize) as u16 ;
            }
            (0x0c,_,_,_) =>{
                let g:u8 = rand::random();
                let result = g & kk;
                self.v[x] = result;
            }
            (0x0d, _, _, _) => {
                // Display Sprite
                self.v[15] = 0;
                for byte in 0..n {
                    let y = (self.v[y] as usize + byte) % 32;
                    for bit in 0..8 {
                        let x = (self.v[x] as usize + bit) % 64;
                        let color = (self.ram[self.i as usize + byte] >> (7 - bit)) & 1;
                        self.v[15] |= color
                            & match self.display[y][x] {
                                true => 1,
                                false => 0,
                            };
                        self.display[y][x] ^= match color {
                            0 => false,
                            1 => true,
                            _ => true,
                        };
                    }
                }
            }
            (0x0e,_,0x0a,0x01) =>{
                self.pc +=2;
            }
            (0x0f,_,0x02,0x09) => {
                self.i = ((self.v[x]) * 5) as u16;
            }
            (0x0f,_,0x03,0x03) =>{
                self.ram[self.i as usize] = self.v[x] / 100;
                self.ram[self.i as usize + 1] = (self.v[x] % 100) / 10;
                self.ram[self.i as usize + 2] = self.v[x] % 10;
            }
            (0x0f,_,0x00,0x07) =>{
                self.delaytime = self.v[x];
            }

            (0x0f,_,0x05,0x05) =>{
                for i in 0..x + 1 {
                    self.ram[(self.i + i as u16) as usize] = self.v[i];
                }
            }
            (0x0f,_,0x06,0x05) =>{
                for i in 0..x + 1 {
                    self.v[i] = self.ram[self.i as usize + i];
                }
            }
            (0x0f,_,0x00,0x0a )=> {
               
            }
            (0x0f,_,0x01,0x0e) =>{
                self.i += self.v[x] as u16;
                self.v[0x0f] = if self.i > 0x0F00 { 1 } else { 0 };
            }
            (0x0f,_,0x01,0x05) =>{
                self.delaytime = self.v[x];     
            }

            _ => {
                println!("ERROR: PRINTING STACK TRACE");

                println!("HALFBYTE: {:?}", halfbyte);
                println!("REG-nnn: {:?}", nnn);
                println!("REG-kk: {:?}", kk);
                println!("REG-y: {:?}", x);
                println!("REG-n: {:?}", y);
                println!("REG-x: {:?}", n);
                println!("PROGRAM_COUNTER: {:?}", self.pc - 2);

                panic!("Error | INVALID OPCODE")
            }
        }
    }
}
