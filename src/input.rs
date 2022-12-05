//TODO: Change this to a bit-mask or something and not 16 varubles, this is a temp solution

use macroquad::input::KeyCode;
use macroquad::prelude::is_key_down;

#[derive(Debug, Copy, Clone)]
pub struct input {
    pub num_1: bool,
    pub num_2: bool,
    pub num_3: bool,
    pub num_4: bool,
    pub num_5: bool,
    pub num_6: bool,
    pub num_7: bool,
    pub num_8: bool,
    pub num_9: bool,
    pub num_0: bool,
    pub num_a: bool,
    pub num_b: bool,
    pub num_c: bool,
    pub num_d: bool,
    pub num_e: bool,
    pub num_f: bool,
}

impl input {
    pub fn new() -> input {
        input {
            num_1: false,
            num_2: false,
            num_3: false,
            num_4: false,
            num_5: false,
            num_6: false,
            num_7: false,
            num_8: false,
            num_9: false,
            num_0: false,
            num_a: false,
            num_b: false,
            num_c: false,
            num_d: false,
            num_e: false,
            num_f: false,
        }
    }
    pub fn if_key_press(&self) -> bool{


        false
    }
    pub fn update(&mut self) {
        self.num_1 = is_key_down(KeyCode::Key1);
        self.num_2 = is_key_down(KeyCode::Key2);
        self.num_3 = is_key_down(KeyCode::Key3);
        self.num_4 = is_key_down(KeyCode::Q);
        self.num_5 = is_key_down(KeyCode::W);
        self.num_6 = is_key_down(KeyCode::E);
        self.num_7 = is_key_down(KeyCode::A);
        self.num_8 = is_key_down(KeyCode::S);
        self.num_9 = is_key_down(KeyCode::D);
        self.num_0 = is_key_down(KeyCode::X);
        self.num_a = is_key_down(KeyCode::Z);
        self.num_b = is_key_down(KeyCode::C);
        self.num_c = is_key_down(KeyCode::Key4);
        self.num_d = is_key_down(KeyCode::R);
        self.num_e = is_key_down(KeyCode::F);
        self.num_f = is_key_down(KeyCode::V);
    }
    pub fn reset(&mut self) {
        self.num_1 = false;
        self.num_2 = false;
        self.num_3 = false;
        self.num_4 = false;
        self.num_5 = false;
        self.num_6 = false;
        self.num_7 = false;
        self.num_8 = false;
        self.num_9 = false;
        self.num_0 = false;
        self.num_a = false;
        self.num_b = false;
        self.num_c = false;
        self.num_d = false;
        self.num_e = false;
        self.num_f = false;
    }
}
