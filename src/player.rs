use Assets;
use shurikens::Shurukens;

use map::Map;
use entity::Entity;

use update::Updateble;

use events::EventListener;
use sdl2::keyboard::Keycode;

use render::Renderable;
use sdl2::render::Renderer;

pub struct Player<'a> {
    pub asset_base: &'a Assets,
    pub ent: Entity<'a>,
    pub shurikens: Shurukens<'a>,

    pub left: bool,
    pub right: bool,

    pub is_jumping: bool, // name is invalid rename to is_jumping
    pub is_grounded: bool,

    pub speed: i32,

    pub jump_buffer: i32,
    pub jump_speed: i32,
    pub gravity: i32,
}

impl<'a> Player<'a> {
    pub fn new(img_src: &Assets, x: i32, y: i32) -> Player {
        Player {
            asset_base: img_src,
            ent: Entity::player(img_src, x, y),
            shurikens: Shurukens::new(),
            left: false,
            right: false,
            is_jumping: false,
            is_grounded: false,
            speed: 6,
            jump_buffer: 0,
            jump_speed: 5,
            gravity: 8,
        }
    }
}

impl<'a> Renderable for Player<'a> {
    fn draw(&self, r: &mut Renderer) {
        self.ent.draw(r);
    }
}

impl<'a> EventListener for Player<'a> {
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
            Keycode::Left => self.left = true,
            Keycode::Right => self.right = true,
            _ => {}
        }
    }
}

impl<'a, 'b> Updateble<&'b Map<'b>> for Player<'a> {
    fn update_with(&mut self, world: &'b Map<'b>) {
        self.ent.dir = if self.left {
            -1
        } else if self.right {
            1
        } else {
            self.ent.dir
        };

        // move left
        if self.left && self.ent.x > 16 {
            if let Some(val) = world.intersect(&self.ent.trans_rect(-self.speed, 0)) {
                self.ent.x = val.right() + 16;
            } else {
                self.ent.x -= self.speed;
                self.ent.anim_next();
            }
        } else if self.left {
            self.ent.x = 16;
        }

        // move right
        if self.right && self.ent.x < 800 - 16 {
            if let Some(val) = world.intersect(&self.ent.trans_rect(self.speed, 0)) {
                self.ent.x = val.left() - 16;
            } else {
                self.ent.x += self.speed;
                self.ent.anim_next();
            }
        } else if self.right {
            self.ent.x = 800 - 16;
        }

        if self.ent.y < 600 - 32 && !self.is_jumping {
            if let Some(val) = world.intersect(&self.ent.trans_rect(0, self.gravity)) {
                self.ent.y = val.top() - 32;
                self.is_grounded = true;
            } else {
                self.ent.y += self.gravity;
                self.is_grounded = false;
            }

        } else if self.jump_buffer > 0 {
            self.jump_buffer -= self.jump_speed;

            if let Some(val) = world.intersect(&self.ent.trans_rect(0, -self.jump_speed)) {
                self.ent.y = val.bottom() + 32;
                self.jump_buffer = 0;
                self.is_jumping = false;
            } else {
                self.ent.y -= self.jump_speed;
            }
        } else {
            self.is_jumping = false;
            self.is_grounded = true; // nes?
        }

        self.shurikens.update_(world, self.ent.get_rect());
    }
}
