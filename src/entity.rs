use sdl2::render::Texture;
use sdl2::rect::Rect;

use assets::*;
use library::*;
use anim::Anim;

#[derive(PartialEq, Clone)]
pub enum EType {
    Player,
    Shuriken,
    Block,
}

#[derive(Clone)]
pub struct Entity<'a> {
    pub m_type: EType,
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub dir: i8,
    pub img: Option<&'a Texture>,

    pub anim: Option<Anim>,
}

impl<'a> Entity<'a> {
    pub fn place_holder(t: EType, img: Option<&Texture>, x: i32, y: i32, w: u32, h: u32) -> Entity {
        Entity {
            m_type: t,
            x: x,
            y: y,
            w: w,
            h: h,
            dir: 0,
            img: img,
            anim: None,
        }
    }

    pub fn player(assets: &Assets, x: i32, y: i32) -> Entity {
        Entity {
            m_type: EType::Player,
            x: x,
            y: y,
            w: 32,
            h: 64,
            dir: 0,
            img: assets.images.get("player"),
            anim: assets.animations.get("player"),
        }
    }

    pub fn shuriken(assets: &Assets, x: i32, y: i32, dir: i8) -> Entity {
        Entity {
            m_type: EType::Shuriken,
            x: x,
            y: y,
            w: 16,
            h: 16,
            dir: dir,
            img: assets.images.get("shuriken"),
            anim: assets.animations.get("shuriken"),
        }
    }

    pub fn block(assets: &Assets, x: i32, y: i32) -> Entity {
        Entity {
            m_type: EType::Block,
            x: x,
            y: y,
            w: 32,
            h: 32,
            dir: 0,
            img: assets.images.get("block"),
            anim: None,
        }
    }

    pub fn anim_frame(&self) -> Option<Rect> {
        match self.anim {
            Some(ref a) => Some(a.r),
            None => Some(Rect::new(0, 0, self.w, self.h)),
        }
    }

    pub fn anim_next(&mut self) {
        match self.m_type {
            EType::Player => {
                let ref mut a = self.anim.as_mut().unwrap();

                if self.dir == -1 {
                    a.next_frame("move_left".to_owned());
                } else if self.dir == 1 {
                    a.next_frame("move_right".to_owned());
                } else {
                    a.next_frame("idle".to_owned());
                }
            }
            EType::Shuriken => {
                let ref mut a = self.anim.as_mut().unwrap();
                a.next_frame("spin".to_owned());
            }
            _ => {}

        }
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(self.x - (self.w / 2) as i32,
                  self.y - (self.h / 2) as i32,
                  self.w,
                  self.h)
    }

    pub fn trans_rect(&self, x: i32, y: i32) -> Rect {
        Rect::new((self.x + x) - (self.w / 2) as i32,
                  (self.y + y) - (self.h / 2) as i32,
                  self.w,
                  self.h)
    }

    pub fn is_offscreen(&self) -> bool {
        let screen_rect = Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);

        return if let Some(_) = screen_rect.intersection(self.get_rect()) {
            false
        } else {
            true
        };
    }
}
