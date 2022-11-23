use macroquad::{
    prelude::{BLACK, RED, WHITE, GRAY, DARKGRAY},
    shapes::{draw_rectangle, draw_line, draw_rectangle_lines},
    window::{clear_background, next_frame, Conf}, text::draw_text, time::get_fps,
};

const WIN_SCALE: u8 = 12;

const CPU_STEP_COUNT:i32 = 16;

mod cpu;
mod font;
mod tests;

fn window_conf() -> Conf {
    Conf {
        window_title    : "Chip8".to_owned(),
        window_height   : (32)* WIN_SCALE as i32,
        window_width    : (64 + 16)* WIN_SCALE as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {


    let mut cpu = cpu::cpu::new();
    cpu.init(include_bytes!("../roms/maze.ch8"));

    loop {
        clear_background(BLACK);

        for i in 0..CPU_STEP_COUNT{
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
            
            // TODO: make sure Gabe never touches this stupid code and make me do stupid changes that do nothing

            for y in 0..33 {
                for x in 0..65 {
                    let x = (x as u32) * WIN_SCALE as u32+1;
                    let y = (y as u32) * WIN_SCALE as u32+1;
    
                    draw_line(x as f32, y  as f32, x as f32, 0 as f32, 1., GRAY);
                    draw_line(x as f32, y  as f32, 0 as f32, y as f32, 1., GRAY);

                }
            }
            
        }


        next_frame().await;
    }

    println!("hello world!");
}
