use macroquad::{
    prelude::{BLACK, DARKGRAY, GRAY, RED, WHITE},
    shapes::{draw_line, draw_rectangle, draw_rectangle_lines},
    text::draw_text,
    time::get_fps,
    window::{clear_background, next_frame, Conf},
};

use std::fs::File;
use std::io::prelude::*;
use std::process::exit;
use std::{env, fs};

const WIN_SCALE: u8 = 12;

const CPU_STEP_COUNT: i32 = 16;

mod cpu;
mod font;
mod input;
mod tests;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chip8".to_owned(),
        window_height: (32) * WIN_SCALE as i32,
        window_width: (64 + 16) * WIN_SCALE as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("Please give path to rom in arguments");
            exit(1)
        }
        2 => {
            //ok we just keep going
        }
        _ => {
            println!("Please give only 1 argument as a argument");
            exit(1)
        }
    }

    let mut f = File::open(&args[1]).unwrap();
    let metadata = fs::metadata(&args[1]).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    let mut cpu = cpu::cpu::new();
    cpu.initv(buffer);

    let mut inputmanager = input::input::new();

    loop {
        clear_background(BLACK);

        inputmanager.update();
        for i in 0..CPU_STEP_COUNT {
            cpu.step();
        }

        for (y, row) in cpu.display.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * WIN_SCALE as u32;
                let y = (y as u32) * WIN_SCALE as u32;

                draw_rectangle(
                    x as f32,
                    y as f32,
                    WIN_SCALE as f32,
                    WIN_SCALE as f32,
                    match col {
                        false => BLACK,
                        true => WHITE,
                    },
                );
            }
        }
        #[cfg(debug_assertions)]
        {
            for y in 0..33 {
                for x in 0..65 {
                    let x = (x as u32) * WIN_SCALE as u32 + 1;
                    let y = (y as u32) * WIN_SCALE as u32 + 1;

                    draw_line(x as f32, y as f32, x as f32, 0 as f32, 1., GRAY);
                    draw_line(x as f32, y as f32, 0 as f32, y as f32, 1., GRAY);
                }
            }
        }

        #[cfg(debug_assertions)]
        {
            for x in 0..4 {
                for y in 0..4 {
                    let toggle: bool = match (4 * y + x) + 1 {
                        1 => inputmanager.num_1,
                        2 => inputmanager.num_2,
                        3 => inputmanager.num_3,
                        4 => inputmanager.num_c,
                        5 => inputmanager.num_4,
                        6 => inputmanager.num_5,
                        7 => inputmanager.num_6,
                        8 => inputmanager.num_d,
                        9 => inputmanager.num_7,
                        10 => inputmanager.num_8,
                        11 => inputmanager.num_9,
                        12 => inputmanager.num_e,
                        13 => inputmanager.num_a,
                        14 => inputmanager.num_0,
                        15 => inputmanager.num_b,
                        16 => inputmanager.num_f,
                        _ => false,
                    };
                    draw_rectangle(
                        850. + (x * 12) as f32 + 1.,
                        50. + (y * 12) as f32 + 1.,
                        11.,
                        11.,
                        match toggle {
                            true => WHITE,
                            false => BLACK,
                        },
                    );
                }
            }
        }

        inputmanager.reset();
        next_frame().await;
    }

    println!("hello world!");
}
