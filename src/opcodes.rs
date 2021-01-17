use processor::CPU;
use graphics::PPU;
use crate::{graphics, processor};

pub fn emulate_cycle(cpu: &mut CPU, ppu: &mut PPU) {
    let highhalf: u16 = cpu.ram[cpu.pc] as u16;
    let high:u16 = (highhalf << 8) | (cpu.ram[cpu.pc + 1] as u16);
    let opcode: u16 = high;

    let operation: u8 = ((opcode & 0xF000) >> 12) as u8;
    let mini_op = opcode & 0x000F;
    //print!("PC = {} : {}\n",cpu.pc, opcode );
    match operation {
        0x0 =>  {
            if opcode == 0x00E0 {
                //clear screen
            }

            else if opcode == 0x00EE {
                cpu.return_from_call();
            }

            else {
                cpu.machine_call();
            }
        },
        0x1 => cpu.jump(),
        0x2 => cpu.call(),
        0x3 => cpu.is_equal_imm(),
        0x4 => cpu.is_not_equal_imm(),
        0x5 => cpu.is_equal_reg(),
        0x6 => cpu.set_reg_to_imm(),
        0x7 => cpu.add_imm_to_reg(),
        0x8 => {
            match mini_op {
                0x0 => cpu.set_reg(),
                0x1 => cpu.bitop_or(),
                0x2 => cpu.bitop_and(),
                0x3 => cpu.bitop_xor(),
                0x4 => cpu.add_reg_to_reg(),
                0x5 => cpu.sub_reg1_to_reg2(),
                0x6 => cpu.bitop_rshift(),
                0x7 => cpu.sub_reg2_to_reg1(),
                0xE => cpu.bitop_lshift(),
                _   => panic!("Bad 8XXX opcode"),
            }
        }
        0x9 => cpu.is_not_equal_reg(),
        0xA => cpu.set_ptr_to_imm(),
        0xB => cpu.jump_to_imm_with_reg(),
        0xC => cpu.rand(),
        0xD => ppu.draw_sprite(cpu),
        0xE => {
            let miniop: u8 = (opcode & 0x00FF) as u8;

            match miniop {
                0x9E => cpu.is_key_pressed(), 
                0xA1 => cpu.is_key_not_pressed(),
                _ => panic!("Bad EXXX opcode!"),
            }
        },
        0xF => {
            let miniop: u8 = (opcode & 0x00FF) as u8;
            
            match miniop {
                0x07 => cpu.set_reg_to_delay(),
                0x0A => cpu.get_key(),
                0x15 => cpu.set_delay_to_reg(),
                0x18 => cpu.set_sound_to_reg(),
                0x1E => cpu.add_reg_to_ptr(),
                0x29 => cpu.set_ptr_to_sprite(),
                0x33 => cpu.set_bcd(),
                0x55 => cpu.reg_dump(),
                0x65 => cpu.reg_load(),
                _ => panic!("Bad FXXX opcode!"),
            }

        }
        _ => panic!("Bad opcode!"),
    }

    cpu.pc += 2;
}