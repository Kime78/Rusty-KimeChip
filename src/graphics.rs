use crate::processor;
use processor::CPU;
use sfml::{
    graphics::{RenderTarget, RenderWindow, Sprite, Texture, Transformable},
    system::Vector2f,
};

pub struct PPU {
    pixels: [u8; 64 * 32 * 4],
}

impl PPU {
    pub fn new() -> Self {
        // //let text = Texture::new(64, 32);
        // //let mut text = text.unwrap();
        // // unsafe {
        // //     text.update_from_pixels(&self.pixels, 64, 32, 0, 0);
        // // }
        // //let text = text.unwrap();
        // let mut spr = Sprite::new();
        // //spr.set_texture(&text, false);

        // spr.set_scale(Vector2f::new(100.0, 400.0));
        // spr.set_position(Vector2f::new(0., 0.));

        Self {
            //sprite: spr,
            pixels: [0; 64 * 32 * 4],
        }
    }

    pub fn draw_sprite(&mut self, cpu: &mut CPU) {
        let highhalf: u16 = cpu.ram[cpu.pc] as u16;
        let high: u16 = (highhalf << 8) | (cpu.ram[cpu.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;
        let y: u8 = ((high & 0x00F0) >> 4) as u8;
        let n: u8 = (high & 0x000F) as u8;
        cpu.regs[0xF] = 1;
        //print!("x: {}, y: {}, n: {}\n", x, y, n);

        for yy in 0..n {
            let data = cpu.ram[(cpu.ptr + yy as u16) as usize];
            for xx in 0..8 {
                if data & (0x80 >> xx) != 0x0 {
                    let cx: u64 = ((cpu.regs[x as usize] + xx) % 64) as u64;
                    let cy: u64 = ((cpu.regs[y as usize] + yy) % 32) as u64;
                    //print!("Data: {}, cx: {}, cy: {}\n", data, cx, cy);
                    // if self.pixels[(cx + cy * 64) as usize] == 255
                    //     && self.pixels[(cx + cy * 64) as usize + 1] == 255
                    //     && self.pixels[(cx + cy * 64) as usize + 2] == 255
                    //     && self.pixels[(cx + cy * 64) as usize + 3] == 255
                    // {
                    //     self.pixels[(cx + cy * 64) as usize] = 0;
                    //     self.pixels[(cx + cy * 64) as usize + 1] = 0;
                    //     self.pixels[(cx + cy * 64) as usize + 2] = 0;
                    //     self.pixels[(cx + cy * 64) as usize + 3] = 255;
                    // } else {
                    self.pixels[(cx * 4 + cy * 64 * 4) as usize] = 255;
                    self.pixels[(cx * 4 + cy * 64 * 4) as usize + 1] = 255;
                    self.pixels[(cx * 4 + cy * 64 * 4) as usize + 2] = 255;
                    self.pixels[(cx * 4 + cy * 64 * 4) as usize + 3] = 255;
                    //xxx += 4;
                    cpu.regs[0xF] = 1;
                    //}
                }
            }

            // for i in 0..32 {
            //     for j in 0..64 {
            //         print!("{} ", self.pixels[j + i * 64]);
            //         print!("{} ", self.pixels[j + i * 64 + 1]);
            //         print!("{} ", self.pixels[j + i * 64 + 2]);
            //         print!("{} ", self.pixels[j + i * 64 + 3]);
            //     }
            //     print!("\n");
            //     print!("\n");
            // }
            // print!("------------------------\n");
            //panic!();
        }
        // let mut i: u64 = 0;
        // while i < (n + 1) as u64 {
        //     let mut j: u64 = 0;
        //     while j < 8 * 4 {
        //         //get the byte
        //         let pixel_byte = cpu.ram[(cpu.ptr + i as u16) as usize];
        //         let pixel = (pixel_byte >> (7 - (j / 4))) & 0x1;

        //         if pixel == 0x1 {
        //             self.pixels[(((i + y as u64) * 64) * 4 + (j + x as u64) * 4) as usize] = 255;
        //             self.pixels[(((i + y as u64) * 64) * 4 + (j + x as u64) * 4 + 1) as usize] =
        //                 255;
        //             self.pixels[(((i + y as u64) * 64) * 4 + (j + x as u64) * 4 + 2) as usize] =
        //                 255;
        //             self.pixels[(((i + y as u64) * 64) * 4 + (j + x as u64) * 4 + 3) as usize] =
        //                 255;
        //         }
        //         // } else {
        //         //     self.pixels[(((i + x as u64) * 64) * 4 + (j + y as u64)) as usize] = 0;
        //         //     self.pixels[(((i + x as u64) * 64) * 4 + (j + y as u64) + 1) as usize] = 0;
        //         //     self.pixels[(((i + x as u64) * 64) * 4 + (j + y as u64) + 2) as usize] = 0;
        //         //     self.pixels[(((i + x as u64) * 64) * 4 + (j + y as u64) + 3) as usize] = 0;
        //         // }

        //         j += 4;
        //     }
        //     i += 1;
        // }

        // for j in 0..(n + 1) {
        //     self.pixels[((i + x) * 32 + (j + y)) as usize] = 255;
        // }
    }

    pub fn draw_frame(&mut self, win: &mut RenderWindow) {
        // self.pixels[(0 * 64) * 4 + 1 * 4 + 0] = 255;
        // self.pixels[(0 * 64) * 4 + 1 * 4 + 1] = 255;
        // self.pixels[(0 * 64) * 4 + 1 * 4 + 2] = 255;
        // self.pixels[(0 * 64) * 4 + 1 * 4 + 3] = 255;

        let text = Texture::new(64, 32);
        let mut text = text.unwrap();
        unsafe {
            text.update_from_pixels(&self.pixels, 64, 32, 0, 0);
        }

        let mut spr = Sprite::new();
        spr.set_texture(&text, false);

        spr.set_scale(Vector2f::new(10.0, 10.0));
        spr.set_position(Vector2f::new(0., 0.));

        win.draw(&spr);
        win.display();
    }
}
