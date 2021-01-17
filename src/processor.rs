use rand::Rng;
use std::u8;
use std::usize;
pub struct CPU {
    pub ram: [u8; 4096 + 0x200],
    pub pc: usize,
    pub ptr: u16,
    regs: [u8; 16],
    stack: [u16; 1024],
    stack_ptr: u8,
    delay: u8,
    sound: u8,
}

impl CPU {
    pub fn new() -> Self {
        let mut tmp: [u8; 4096 + 0x200] = [0; 4096 + 0x200];
        let sprites: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        for i in 0..80 {
            tmp[i] = sprites[i];
        }

        Self {
            ram: tmp,
            pc: 0x200,
            regs: [0; 16],
            stack: [0; 1024],
            stack_ptr: 0,
            ptr: 0,
            delay: 0,
            sound: 0,
        }
    }

    pub fn machine_call(&mut self) {
        panic!("ita");
        // self.stack[self.stack_ptr as usize] = ((self.pc << 8) | self.pc) as u16;
        // self.stack_ptr += 1;
    }

    pub fn call(&mut self) {
        self.stack[self.stack_ptr as usize] = ((self.pc << 8) | (self.pc + 1)) as u16;
        self.stack_ptr += 1;
    }

    pub fn return_from_call(&mut self) {
        self.pc = self.stack[self.stack_ptr as usize] as usize;
        self.stack_ptr -= 1;
    }

    pub fn jump(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);

        self.pc = high as usize & 0x0FFFusize; //endiness
        self.pc -= 2;
    }

    pub fn is_equal_imm(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;
        let imm: u8 = (high & 0x00FF) as u8;

        if self.regs[x as usize] == imm {
            self.pc += 2;
        }
    }

    pub fn is_not_equal_imm(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;
        let imm: u8 = (high & 0x00FF) as u8;

        if self.regs[x as usize] != imm {
            self.pc += 2;
        }
    }

    pub fn is_equal_reg(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;
        let y: u8 = ((high & 0x00F0) >> 4) as u8;

        if self.regs[x as usize] == self.regs[y as usize] {
            self.pc += 2;
        }
    }

    pub fn is_not_equal_reg(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;
        let y: u8 = ((high & 0x00F0) >> 4) as u8;

        if self.regs[x as usize] != self.regs[y as usize] {
            self.pc += 2;
        }
    }

    pub fn set_reg_to_imm(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;
        let imm: u8 = (high & 0x00FF) as u8;

        self.regs[x as usize] = imm;
    }

    pub fn add_imm_to_reg(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;
        let imm: u8 = (high & 0x00FF) as u8;

        let res: (u8, bool) = self.regs[x as usize].overflowing_add(imm);
        self.regs[x as usize] = res.0;
    }

    pub fn set_reg(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;
        let y: u8 = ((high & 0x00F0) >> 4) as u8;

        self.regs[x as usize] = self.regs[y as usize];
    }

    pub fn bitop_or(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;
        let y: u8 = ((high & 0x00F0) >> 4) as u8;

        self.regs[x as usize] |= self.regs[y as usize];
    }

    pub fn bitop_and(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;
        let y: u8 = ((high & 0x00F0) >> 4) as u8;

        self.regs[x as usize] &= self.regs[y as usize];
    }

    pub fn bitop_xor(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;
        let y: u8 = ((high & 0x00F0) >> 4) as u8;

        self.regs[x as usize] ^= self.regs[y as usize];
    }

    pub fn bitop_rshift(&mut self) {
        //rename me
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;

        self.regs[0xF] = self.regs[x as usize] & 0x1;
        self.regs[x as usize] >>= 1;
    }

    pub fn bitop_lshift(&mut self) {
        //rename me
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;

        self.regs[0xF] = (self.regs[x as usize] >> 4) & 0x80;
        self.regs[x as usize] <<= 1;
    }

    pub fn add_reg_to_reg(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;
        let y: u8 = ((high & 0x00F0) >> 4) as u8;

        let res: (u8, bool) = self.regs[x as usize].overflowing_add(self.regs[y as usize]);
        self.regs[x as usize] = res.0;
        self.regs[0xF as usize] = res.1 as u8;
    }

    pub fn sub_reg1_to_reg2(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;
        let y: u8 = ((high & 0x00F0) >> 4) as u8;

        let res: (u8, bool) = self.regs[x as usize].overflowing_sub(self.regs[y as usize]);
        self.regs[x as usize] = res.0;
        self.regs[0xF as usize] = res.1 as u8;
    }

    pub fn sub_reg2_to_reg1(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;
        let y: u8 = ((high & 0x00F0) >> 4) as u8;

        let res: (u8, bool) = self.regs[y as usize].overflowing_sub(self.regs[x as usize]);
        self.regs[x as usize] = res.0;
        self.regs[0xF as usize] = res.1 as u8;
    }

    pub fn set_ptr_to_imm(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        self.ptr = high & 0x0FFF;
    }

    pub fn jump_to_imm_with_reg(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        self.pc = high as usize & 0x0FFF + self.regs[0] as usize;
    }

    pub fn rand(&mut self) {
        let secret_number = rand::thread_rng().gen_range(0, 255);
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;
        let imm: u8 = (high & 0x00FF) as u8;

        self.regs[x as usize] = secret_number & imm;
    }

    pub fn is_key_pressed(&mut self) {
        panic!("ita1");
    }

    pub fn is_key_not_pressed(&mut self) {
        panic!("ita2");
    }

    pub fn get_key(&mut self) {
        panic!("ita3");
    }

    pub fn set_reg_to_delay(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;

        self.regs[x as usize] = self.delay;
    }

    pub fn set_sound_to_reg(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;

        self.sound = self.regs[x as usize];
    }

    pub fn set_delay_to_reg(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;

        self.delay = self.regs[x as usize];
    }

    pub fn set_ptr_to_sprite(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;

        self.ptr = (self.regs[x as usize] * 5) as u16;
    }

    pub fn add_reg_to_ptr(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;

        self.ptr = self.regs[x as usize] as u16;
    }

    pub fn set_bcd(&mut self) {
        let highhalf: u16 = self.ram[self.pc] as u16;
        let high: u16 = (highhalf << 8) | (self.ram[self.pc + 1] as u16);
        let mut digit: u8 = ((high & 0x0F00) >> 8) as u8;

        let mut index = 3;
        while index > 0 {
            self.ram[(self.ptr + index - 1) as usize] = digit % 10;
            digit /= 10;
            index -= 1;
        }
    }

    pub fn reg_dump(&mut self) {
        for i in 0..15 {
            self.ram[self.ptr as usize + i] = self.regs[i];
        }
    }

    pub fn reg_load(&mut self) {
        for i in 0..15 {
            self.regs[i] = self.ram[self.ptr as usize + i]
        }
    }
}
