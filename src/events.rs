use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use Player;
use map::Map;
use Entity;
use sdl2::rect::Rect;

pub trait EventListener {
    fn key_up(&mut self, _: Keycode);
    fn key_down(&mut self, _: Keycode);

    fn mouse_up(&mut self, _: MouseButton, _: i32, _: i32);
    fn mouse_down(&mut self, _: MouseButton, _: i32, _: i32);
}

impl<'a> EventListener for Player<'a> {
    fn mouse_up(&mut self, _: MouseButton, _: i32, _: i32) {}
    fn mouse_down(&mut self, _: MouseButton, _: i32, _: i32) {}

    fn key_up(&mut self, k: Keycode) {
        match k {
            Keycode::Left => {
                self.left = false;
            }
            Keycode::Right => {
                self.right = false;
            }
            _ => {}
        }
    }

    fn key_down(&mut self, k: Keycode) {
        match k {
            Keycode::Space => {
                if !self.is_jumping && self.is_grounded {
                    self.is_jumping = true;
                    self.jump_buffer = 32 * 3;
                }
            }
            Keycode::Z => {
                
                    if self.ent.dir != 0 {
                        let x = self.ent.x + (self.ent.dir as i32 * (16 + 8 + 1));

                        self.shurikens.throw(self.asset_base, self.ent.dir, x, self.ent.y);

                        /*if let Some(_) = world.intersect(&Rect::new(x, player.ent.y, 16, 16)) {
                        } else {
                            let t_shir =
                                Entity::shuriken(&m_assets, x, self.ent.y, self.ent.dir);

                            // shir.push(t_shir);

                            self.shurikens.items.push(t_shir);
                        }*/
                    }
            }
            Keycode::Left => {
                self.left = true;
            }
            Keycode::Right => {
                self.right = true;
            }
            _ => {}
        }
    }
}

impl<'a> EventListener for Map<'a> {
    fn key_up(&mut self, _: Keycode) {}
    fn key_down(&mut self, _: Keycode) {}

    fn mouse_up(&mut self, btn: MouseButton, _: i32, _: i32) {
        match btn {
            MouseButton::Left => self.can_place = true,
            MouseButton::Right => self.can_remove = true,
            _ => {}
        }
    }

    fn mouse_down(&mut self, btn: MouseButton, x: i32, y: i32) {
        match btn {
            MouseButton::Left => {
                if self.can_place {
                    /*
                    let re = x % 32;
                    if re != 0 {
                        if x - re > 16 {
                            x = x - re;
                        } else {
                            x = x + re;
                        }
                    }
                    let re = y % 32;
                    if re != 0 {
                        if y - re > 16 {
                            y = y - re;
                        } else {
                            y = y + re;
                        }
                    }*/

                    if let None = self.intersect(&Rect::new(x - 16, y - 16, 32, 32)) {
                        let t_block = Entity::block(&self.img_s, x, y);
                        self.items.push(t_block);
                    }
                }
            }
            MouseButton::Right => {
                if self.can_remove {
                    if let Some(id) = self.clicked(x, y) {
                        self.items.remove(id);
                    }

                    self.can_remove = false;
                }
            }
            _ => {} 
        }
    }
}