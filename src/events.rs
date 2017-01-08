use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

pub trait EventListener {
    fn key_up(&mut self, _: Keycode) {}
    fn key_down(&mut self, _: Keycode) {}

    fn mouse_up(&mut self, _: MouseButton, _: i32, _: i32) {}
    fn mouse_down(&mut self, _: MouseButton, _: i32, _: i32) {}
}