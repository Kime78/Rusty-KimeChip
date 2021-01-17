use crate::processor;
use processor::CPU;
use sfml::{
    graphics::{RenderTarget, RenderWindow, Sprite, Texture, Transformable},
    system::Vector2f,
};

pub struct PPU {
    pixels: [u8; 64 * 100 * 4],
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
            pixels: [0; 64 * 100 * 4],
        }
    }

    pub fn draw_sprite(&mut self, cpu: &mut CPU) {
        let highhalf: u16 = cpu.ram[cpu.pc] as u16;
        let high: u16 = (highhalf << 8) | (cpu.ram[cpu.pc + 1] as u16);
        let x: u8 = ((high & 0x0F00) >> 8) as u8;
        let y: u8 = ((high & 0x00F0) >> 4) as u8;
        let n: u8 = (high & 0x000F) as u8;
        //print!("x: {}, y: {}, n: {}\n", x, y, n);
        let mut i: u64 = 0;
        while i < (n + 1) as u64 {
            let mut j: u64 = 0;
            while j < 8 * 4 {
                if cpu.ram[(i * 32 + j + cpu.ptr as u64) as usize] == 1 {
                    self.pixels[(((i + x as u64) * 64) * 4 + (j + y as u64) * 4) as usize] = 0;
                    self.pixels[(((i + x as u64) * 64) * 4 + (j + y as u64) * 4 + 1) as usize] =
                        255;
                    self.pixels[(((i + x as u64) * 64) * 4 + (j + y as u64) * 4 + 2) as usize] = 0;
                    self.pixels[(((i + x as u64) * 64) * 4 + (j + y as u64) * 4 + 3) as usize] =
                        255;
                } else {
                    self.pixels[(((i + x as u64) * 64) * 4 + (j + y as u64)) as usize] = 0;
                    self.pixels[(((i + x as u64) * 64) * 4 + (j + y as u64) + 1) as usize] = 0;
                    self.pixels[(((i + x as u64) * 64) * 4 + (j + y as u64) + 2) as usize] = 0;
                    self.pixels[(((i + x as u64) * 64) * 4 + (j + y as u64) + 3) as usize] = 255;
                }

                j += 4;
            }
            i += 1;
        }

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
