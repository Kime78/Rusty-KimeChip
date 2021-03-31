use graphics::PPU;
use opcodes::emulate_cycle;
use processor::CPU;
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{Event, Style},
};
use std::io;
use std::io::prelude::*;
use std::process::exit;
mod graphics;
mod keypad;
mod opcodes;
mod processor;

fn main() {
    let mut cpu: CPU = CPU::new();
    let mut ppu: PPU = PPU::new();
    let mut gae: [u8; 4096] = [0; 4096];

    println!("Selectati ce joc doriti sa jucati din lista de mai jos:");
    println!("1. Tetris");
    println!("2. Brix");
    println!("3. Pong");
    println!("4. Merlin");
    println!("5. KaleID");
    //let mut path = String::new();
    let mut game = String::new();

    io::stdin()
        .read_line(&mut game)
        .expect("Failed to read line");
    let game: u32 = game
        .trim()
        .parse()
        .expect("Caracterul introdus nu este valid");

    if game == 1 {
        println!("Doriti instructiuni pentru joc? [D / N]");
        let mut instructiuni = String::new();
        io::stdin()
            .read_line(&mut instructiuni)
            .expect("Failed to read line");

        if instructiuni.chars().nth(0).expect("idfk") == 'D'
            || instructiuni.chars().nth(0).expect("idfk") == 'd'
        {
            println!("Q - Roteste Piesa\nW si E - Deplaseaza piesa stanga-dreapta");
            println!("Apasati Enter pentru a juca");
            let mut any = String::new();
            io::stdin()
                .read_line(&mut any)
                .expect("Failed to read line");
        }

        let mut file_in = std::fs::File::open("./roms/tetris").unwrap();
        file_in.read(&mut gae).unwrap();
    } else if game == 2 {
        println!("Doriti instructiuni pentru joc? [D / N]");
        let mut instructiuni = String::new();
        io::stdin()
            .read_line(&mut instructiuni)
            .expect("Failed to read line");

        if instructiuni.chars().nth(0).expect("idfk") == 'D'
            || instructiuni.chars().nth(0).expect("idfk") == 'd'
        {
            println!("Q si E - Deplaseaza paleta stanga-dreapta");
            println!("Apasati Enter pentru a juca");
            let mut any = String::new();
            io::stdin()
                .read_line(&mut any)
                .expect("Failed to read line");
        }

        let mut file_in = std::fs::File::open("./roms/brix").unwrap();
        file_in.read(&mut gae).unwrap();
    } else if game == 3 {
        println!("Doriti instructiuni pentru joc? [D / N]");
        let mut instructiuni = String::new();
        io::stdin()
            .read_line(&mut instructiuni)
            .expect("Failed to read line");

        if instructiuni.chars().nth(0).expect("idfk") == 'D'
            || instructiuni.chars().nth(0).expect("idfk") == 'd'
        {
            println!("1 si Q - Deplaseaza paleta din stanga sus-jos");
            println!("4 si R - Deplaseaza paleta din dreapta sus-jos");
            println!("Apasati Enter pentru a juca");
            let mut any = String::new();
            io::stdin()
                .read_line(&mut any)
                .expect("Failed to read line");
        }

        let mut file_in = std::fs::File::open("./roms/pong").unwrap();
        file_in.read(&mut gae).unwrap();
    } else if game == 4 {
        println!("Doriti instructiuni pentru joc? [D / N]");
        let mut instructiuni = String::new();
        io::stdin()
            .read_line(&mut instructiuni)
            .expect("Failed to read line");

        if instructiuni.chars().nth(0).expect("idfk") == 'D'
            || instructiuni.chars().nth(0).expect("idfk") == 'd'
        {
            println!("Q - Apasa Butonul din coltul stanga-sus");
            println!("W - Apasa Butonul din coltul dreapta-sus");
            println!("A - Apasa Butonul din coltul stanga-jos");
            println!("S - Apasa Butonul din coltul dreapta-jos");

            println!("Apasati Enter pentru a juca");
            let mut any = String::new();
            io::stdin()
                .read_line(&mut any)
                .expect("Failed to read line");
        }
        let mut file_in = std::fs::File::open("./roms/merlin").unwrap();
        file_in.read(&mut gae).unwrap();
    } else if game == 5 {
        println!("Doriti instructiuni pentru joc? [D / N]");
        let mut instructiuni = String::new();
        io::stdin()
            .read_line(&mut instructiuni)
            .expect("Failed to read line");

        if instructiuni.chars().nth(0).expect("idfk") == 'D'
            || instructiuni.chars().nth(0).expect("idfk") == 'd'
        {
            println!("Demo de generare a unor pattern-uri simetrice");
            println!("Q si E - Deplasarea liniei stanga-dreapta");
            println!("2 si S - Deplasarea liniei sus-jos");
            println!("Apasati Enter pentru a juca");
            let mut any = String::new();
            io::stdin()
                .read_line(&mut any)
                .expect("Failed to read line");
        }
        let mut file_in = std::fs::File::open("./roms/kaleid").unwrap();
        file_in.read(&mut gae).unwrap();
    } else {
        panic!("Numarul introdus nu este valid, introduce-ti un numar de la 1 la 5");
        //do a repetition
    }
    // for argument in env::args() {
    //     println!("{}", argument);
    //     //path = argument;
    // }

    // print!("{}", z);

    for i in 0..4096 {
        cpu.ram[0x200 + i] = gae[i];
    }
    let mut window = RenderWindow::new((640, 320), "Rusty Chip", Style::CLOSE, &Default::default());

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
