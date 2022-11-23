use core::panic;


use log::{info};
use macroquad::{
    audio::{load_sound_from_bytes, play_sound, PlaySoundParams},
    prelude::{BLACK, WHITE},
    shapes::draw_rectangle,
    window::{next_frame, Conf},
};
use rand::random;
mod font;

const WIN_SCALE: u8 = 12;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chip8".to_owned(),
        window_height: 32 * WIN_SCALE as i32,
        window_width:  16 + 64 * WIN_SCALE as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut display = [[false; 64]; 32];

    let mut vm_RAM: [u8; 4096] = [0; 4096];
    let mut vm_PC: u16 = 0x198;
    let mut vm_I: u16 = 0;
    let mut vm_stack: [usize; 256] = [0; 256];
    let mut vm_sp: usize = 0;
    let mut vm_delaytimer: u8 = 0;
    let mut vm_soundtimer: u8 = 2; // TODO: Make this do beep

    //Registers(messy code)

    let mut vm_vn: [u8; 16] = [0; 16];

    //Inject Font
    let mut findex: usize = 0;
    for font_byte in font::FONT_MEM.iter() {
        vm_RAM[0x050 + findex] = font_byte.clone();
        findex = findex + 1;
    }
    let mut pindex: usize = 0;
    for ibm_byte in include_bytes!("../roms/blinky.ch8") {
        vm_RAM[0x200 + pindex] = ibm_byte.clone();
        pindex = pindex + 1;
    }

    //Inject IBM TODO: Remove this in the future\

    //TODO: Redo all this code, its is so slow and bloated 

    loop {
        //let byte1 = 169 as u8; //vm_RAM[vm_PC as usize];
        //let byte2 = vm_RAM[vm_PC as usize + 1];
        vm_PC = vm_PC + 2;

        let opcode = (vm_RAM[vm_PC as usize] as u16) << 8 | (vm_RAM[vm_PC as usize + 1] as u16);
        let halfbyte = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );
        let nnn = (opcode & 0x0FFF) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let x = halfbyte.1 as usize;
        let y = halfbyte.2 as usize;
        let n = halfbyte.3 as usize;

        match halfbyte {
            (0x00, 0x00, 0x0e, 0x00) => {
                //Clear Screen
                display = [[false; 64]; 32];
            }
            (0x01, _, _, _) => {
                // jump
                vm_PC = nnn as u16 - 2;
                info!("jumping to {}", nnn)
            }
            (0x06, _, _, _) => {
                //Set Register VX
                vm_vn[x] = kk;
            }
            (0x07, _, _, _) => {
                //Set Register VX
                vm_vn[x] = vm_vn[x].wrapping_add(kk)
            }
            (0x0a, _, _, _) => {
                //Set Register VX
                vm_I = nnn as u16;
            }
            (0x0d, _, _, _) => {
                //Set Register VX
                vm_vn[15] = 0;
                for byte in 0..n {
                    let y = (vm_vn[y] as usize + byte) % 32;
                    for bit in 0..8 {
                        let x = (vm_vn[x] as usize + bit) % 64;
                        let color = (vm_RAM[vm_I as usize + byte] >> (7 - bit)) & 1;
                        vm_vn[15] |= color
                            & match display[y][x] {
                                true => 1,
                                false => 0,
                            };
                        display[y][x] ^= match color {
                            0 => false,
                            1 => true,
                            _ => true,
                        };
                    }
                }
            }

            (0x00, 0x00, 0x0e, 0x0e) => {
                vm_sp -= 1;
                vm_PC = vm_stack[vm_sp] as u16;
            }

            (0x00, _, _, _) => {
                //TODO: Impliment Mechine Lanuguage Simulator at a later date
            }

            (0x02, _, _, _) => {
                vm_stack[vm_sp] = vm_PC as usize + 2;
                vm_sp += 1;
            }
            (0x3, _, _, _) => {
                if vm_vn[x] == kk {
                    vm_PC = vm_PC + 2;
                    info!("jumping to {}", vm_PC)
                }
            }
            (0x4, _, _, _) => {
                if vm_vn[x] != kk {
                    vm_PC = vm_PC + 2;
                    info!("jumping to {}", vm_PC)
                }
            }
            (0x5, _, _, 0x00) => {
                if vm_vn[x] == vm_vn[y] {
                    vm_PC = vm_PC + 2;
                    info!("jumping to {}", vm_PC)
                }
            }
            (0x5, _, _, 0x09) => {
                if vm_vn[x] != vm_vn[y] {
                    vm_PC = vm_PC + 2;
                    info!("jumping to {}", vm_PC)
                }
            }

            //Logic stuff comming up
            (0x08, _, _, 0x00) => vm_vn[x] = vm_vn[y],
            (0x08, _, _, 0x01) => vm_vn[x] = vm_vn[x] | vm_vn[y],
            (0x08, _, _, 0x02) => vm_vn[x] = vm_vn[x] & vm_vn[y],
            (0x08, _, _, 0x03) => vm_vn[x] = vm_vn[x] ^ vm_vn[y],
            (0x08, _, _, 0x04) => vm_vn[x] = vm_vn[x].wrapping_add(vm_vn[y]),
            (0x08, _, _, 0x05) => {
                vm_vn[15] = if vm_vn[x] > vm_vn[y] { 1 } else { 0 };
                vm_vn[x] = vm_vn[x].wrapping_sub(vm_vn[y]);
            }
            (0x08, _, _, 0x07) => {
                vm_vn[0x0f] = if vm_vn[y] > vm_vn[x] { 1 } else { 0 };
                vm_vn[x] = vm_vn[y].wrapping_sub(vm_vn[x]);
            }
            (0x08, _, _, 0x06) => {
                // TODO: AMBIGOUS
                vm_vn[15] = vm_vn[x] & 1;
                vm_vn[x] >>= 1;
            }
            (0x08, _, _, 0x0E) => {
                // TODO: AMBIGOUS
                vm_vn[15] = (vm_vn[x] & 0b10000000) >> 7;
                vm_vn[x] <<= 1;
            }
            (0x0b, _, _, _) => {
                // TODO: Ambisuous
                vm_PC = (vm_vn[0] as u16) + nnn as u16;
                info!("jumping to {}", vm_PC)
            }
            (0x0c, _, _, _) => {
                let randx: u8 = random();
                vm_vn[x as usize] = randx & kk;
            }
            (0x0e, _, 0x09, 0x0e) => {
                //TODO ADD KEYBOARD SUPPORT
                panic!("No Keyboard Support");
                vm_PC = vm_PC + 2;
            }
            (0x0e, _, 0x0a, 0x01) => {
                //panic!("No Keyboard support")
                //vm_PC = vm_PC + 2;
            }
            (0x0F, _, 0x00, 0x07) => {
                vm_vn[x as usize] = vm_delaytimer;
            }
            (0x0F, _, 0x01, 0x05) => {
                vm_delaytimer = vm_vn[x as usize];
            }
            (0x0F, _, 0x01, 0x08) => {
                vm_soundtimer = vm_vn[x as usize];
            }
            (0x0f, _, 0x01, 0x0e) => {
                vm_I = vm_I + vm_vn[x as usize] as u16;
            }
            (0x0f, _, 0x00, 0x0a) => {
                //TODO: No Keyboard Yet
                //panic!("no keyboard");
            }
            (0x0f, _, 0x06, 0x05) => {
                for i in 0..x + 1 {
                    vm_vn[i] = vm_RAM[vm_I as usize + i];
                }
            }
            (0x0f, _, 0x05, 0x05) => {
                for i in 0..x + 1 {
                    vm_RAM[(vm_I + i as u16) as usize] = vm_vn[i];
                }
            }
            (0x09, _, _, 0) => {
                if vm_vn[x] != vm_vn[y] {
                    vm_PC = vm_PC + 1;
                }
            }
            (0x0f, _, 0x03, 0x03) => {
                vm_RAM[vm_I as usize] = vm_vn[x] / 100;
                vm_RAM[(vm_I + 1) as usize] = (vm_vn[x] % 100) / 10;
                vm_RAM[(vm_I + 2) as usize] = vm_vn[x] % 10;
            }
            (0x0f, _, 0x02, 0x09) => {
                vm_I = ((vm_vn[x] as usize) * 5) as u16;
            }

            _ => {
                panic!("unkown opcode")
            } //No oporation
        }

        //Sound Stuff
        if vm_delaytimer > 0 {
            vm_delaytimer = vm_delaytimer - 1;
        }
        if vm_soundtimer > 0 {
            //let sound = load_sound_from_bytes(include_bytes!("beep.wav"))
            //    .await
            //    .unwrap();
            //play_sound(
            //    sound,
            //    PlaySoundParams {
            //        looped: false,
            //        volume: 1.,
            //    },
            //);
            vm_soundtimer = vm_soundtimer - 1;
        }

        //let mut posx = 0.0;
        //let mut posy = 0.0;
        for (y, row) in display.iter().enumerate() {
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

        next_frame().await
    }
}
