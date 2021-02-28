use opcodes::emulate_cycle;
use processor::CPU;
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{Event, Style},
};
use std::io::prelude::*;
mod graphics;
mod processor;
use graphics::PPU;
mod keypad;
mod opcodes;
use std::process::exit;
fn main() {
    let mut window = RenderWindow::new((640, 320), "Rusty Chip", Style::CLOSE, &Default::default());

    let mut cpu: CPU = CPU::new();
    let mut ppu: PPU = PPU::new();
    let mut gae: [u8; 4096] = [0; 4096];
    let mut file_in = std::fs::File::open("./roms/delay").unwrap();
    file_in.read(&mut gae).unwrap();
    // print!("{}", z);

    for i in 0..4096 {
        cpu.ram[0x200 + i] = gae[i];
    }

    window.set_framerate_limit(60);

    loop {
        emulate_cycle(&mut cpu, &mut ppu, &mut window);
        //ppu.draw_frame(&mut window);
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                exit(0);
            }
        }
        window.clear(Color::BLACK);
    }

    //window.set_active(true);
}
