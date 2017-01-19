use events::*;

pub struct Camera {
    pub scale: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera { scale: 1.0 }
    }
}

impl EventListener for Camera {
    fn key_down(&mut self, key: Keycode) {
        match key {
            Keycode::W => {
                self.scale += 0.05;
            }
            Keycode::S => {
                self.scale -= 0.05;
            }
            _ => {}
        }
    }
}