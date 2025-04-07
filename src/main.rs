use std::time::Duration;

use nes_rust::{cpu::{snake, CPU}, format_test::trace, rom::Rom, MemAccess};
use rand::Rng;
use sdl2::{event::Event, keyboard::Keycode, pixels::{Color, PixelFormatEnum}, EventPump};

// This code block will run the snake program
// fn main() {
//     let snake_program = snake::snake_program();
//     let mut cpu = CPU::new();

//     let sdl_context = sdl2::init().unwrap();
//     let video_sys = sdl_context.video().unwrap();
//     let window = video_sys
//         .window("Snake game", 320, 320)
//         .position_centered()
//         .build().unwrap();

//     let mut canvas = window.into_canvas().present_vsync().build().unwrap();
//     let mut event_pump = sdl_context.event_pump().unwrap();
//     canvas.set_scale(10.0, 10.0).unwrap();

//     let creator = canvas.texture_creator();
//     let mut texture = creator
//         .create_texture_target(PixelFormatEnum::RGB24, 32, 32)
//         .unwrap();

//     let mut screen_state = [0 as u8; 32 * 32 * 3];
//     let mut rng = rand::thread_rng();

//     cpu.load_snake();
//     cpu.reset();
//     cpu.run_with_callback(move |cpu| {
//         handle_user_input(cpu, &mut event_pump);
//         cpu.mem_write(0xFE, rng.gen_range(1, 16));

//         if read_screen_state(cpu, &mut screen_state) {
//             texture.update(None, &screen_state, 32 * 3).unwrap();
//             canvas.copy(&texture, None, None).unwrap();
//             canvas.present();
//         }

//         std::thread::sleep(Duration::from_nanos(70_000));
//     });
// }

// This code block is used for test rom logging
fn main() {
    let mut cpu = CPU::new();
    cpu.load_rom(Rom::from_rom("./nestest.nes").unwrap());
    cpu.reset();
    cpu.run_with_callback(|cpu| {
        println!("{}", trace(cpu));
    });
}

fn handle_user_input(cpu: &mut CPU, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                std::process::exit(0);
            },
            Event::KeyDown { keycode: Some(Keycode::W), ..} => {
                cpu.mem_write(0xFF, 0x77);
            },
            Event::KeyDown { keycode: Some(Keycode::S), ..} => {
                cpu.mem_write(0xFF, 0x73);
            },
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                cpu.mem_write(0xFF, 0x61);
            },
            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                cpu.mem_write(0xFF, 0x64);
            },
            _ => () // do nothing
        }
    }
}

fn color(byte: u8) -> Color {
    match byte {
        0 => Color::BLACK,
        1 => Color::WHITE,
        2 | 9 => Color::GRAY,
        3 | 10 => Color::RED,
        4 | 11 => Color::GREEN,
        5 | 12 => Color::BLUE,
        6 | 13 => Color::MAGENTA,
        7 | 14 => Color::YELLOW,
        _ => Color::CYAN,
    }
}

fn read_screen_state(cpu: &CPU, frame: &mut [u8; 32 * 32 * 3]) -> bool {
    let mut frame_idx = 0;
    let mut update = false;
    for i in 0x200..0x600 {
        let color_idx = cpu.mem_read(i as u16);
        let (b1, b2, b3) = color(color_idx).rgb();
        if frame[frame_idx] != b1 || frame[frame_idx + 1] != b2 || frame[frame_idx + 2] != b3 {
            frame[frame_idx] = b1;
            frame[frame_idx + 1] = b2;
            frame[frame_idx + 2] = b3;
            update = true;
        }
        frame_idx += 3;
    }
    update
}
