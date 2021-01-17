use opcodes::emulate_cycle;
use processor::CPU;
use sfml::{graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture, Transformable}, system::{Vector2f}, window::{Event, Style}};
use std::io::prelude::*;
mod processor;
mod graphics;
use graphics::PPU;
mod opcodes;
fn main() {
    let mut window = RenderWindow::new((640, 320), "Rusty Chip", Style::CLOSE, &Default::default());
    
    let mut cpu: CPU = CPU::new();
    let mut ppu: PPU = PPU::new();
    let mut gae: [u8; 4096] = [0; 4096];  
    let mut file_in = std::fs::File::open("./roms/boncoder").unwrap();
    file_in.read(&mut gae).unwrap();
    for i in 0..4096 {
        cpu.ram[0x200 + i] = gae[i];
    }

    window.set_framerate_limit(60);

    while window.is_open() {
        emulate_cycle(&mut cpu, &mut ppu);
        ppu.draw_frame(&mut window);
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }
        }
        window.clear(Color::BLACK);
    }

    //window.set_active(true);
    
}