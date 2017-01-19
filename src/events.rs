pub use sdl2::keyboard::Keycode;
pub use sdl2::mouse::MouseButton;

pub struct Listener {
    pub listeners: Vec<*mut EventListener> 
}

impl Listener {
    pub fn new() -> Listener {
        Listener {
            listeners: vec![],
        }
    }
}

pub trait EventListener {
    fn key_up(&mut self, _: Keycode) {}
    fn key_down(&mut self, _: Keycode) {}

    fn mouse_up(&mut self, _: MouseButton, _: i32, _: i32) {}
    fn mouse_down(&mut self, _: MouseButton, _: i32, _: i32) {}
}

impl EventListener for Listener {
    fn key_up(&mut self, k: Keycode) {
        for i in 0..self.listeners.len() {
            unsafe {
                (*self.listeners[i]).key_up(k);
            }
        }
    }
    fn key_down(&mut self, k: Keycode) {
        for i in 0..self.listeners.len() {
            unsafe {
                (*self.listeners[i]).key_down(k);
            }
        }
    }
    fn mouse_up(&mut self, mb: MouseButton, x: i32, y: i32) {
        for i in 0..self.listeners.len() {
            unsafe {
                (*self.listeners[i]).mouse_up(mb,x,y);
            }
        }
    }
    fn mouse_down(&mut self, mb: MouseButton, x: i32, y: i32) {
        for i in 0..self.listeners.len() {
            unsafe {
                (*self.listeners[i]).mouse_up(mb,x,y);
            }
        }
    }
}