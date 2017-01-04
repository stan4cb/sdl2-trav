use sdl2::render::{Renderer, Texture};
use sdl2::rect::Rect;
use sdl2::pixels::Color;

use assets::*;
use anim::Anim;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

#[derive(PartialEq)]
pub enum EType {
    Player,
    Shuriken,
    Block,
}

pub struct Entity<'a> {
    pub m_type: EType,
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub dir: i8,
    img: Option<&'a Texture>,

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

    pub fn player(src_img: &ImageS, x: i32, y: i32) -> Entity {
        Entity {
            m_type: EType::Player,
            x: x,
            y: y,
            w: 32,
            h: 64,
            dir: 0,
            img: src_img.get_image("player"),
            anim: Some(Anim::new_with_r(Rect::new(0, 0, 32, 64),
                                        &vec![("idle".to_owned(),
                                               vec![Rect::new(0, 0, 32, 64),
                                                    Rect::new(32, 0, 32, 64),
                                                    Rect::new(64, 0, 32, 64)]),
                                              ("move_left".to_owned(),
                                               vec![Rect::new(0, 64, 32, 64),
                                                    Rect::new(32, 64, 32, 64),
                                                    Rect::new(64, 64, 32, 64)]),
                                              ("move_right".to_owned(),
                                               vec![Rect::new(0, 128, 32, 64),
                                                    Rect::new(32, 128, 32, 64),
                                                    Rect::new(64, 128, 32, 64)])])),
        }
    }
    pub fn shuriken(src_img: &ImageS, x: i32, y: i32, dir: i8) -> Entity {
        Entity {
            m_type: EType::Shuriken,
            x: x,
            y: y,
            w: 16,
            h: 16,
            dir: dir,
            img: src_img.get_image("shuriken"),
            anim: Some(Anim::new_with_r(Rect::new(0, 0, 16, 16),
                                        &vec![("spin".to_owned(),
                                               vec![Rect::new(0, 0, 16, 16),
                                                    Rect::new(16, 0, 16, 16)])])),
        }
    }

    pub fn block(src_img: &ImageS, x: i32, y: i32) -> Entity {
        Entity {
            m_type: EType::Block,
            x: x,
            y: y,
            w: 32,
            h: 32,
            dir: 0,
            img: src_img.get_image("block"),
            anim: None,
        }
    }

    pub fn gen_clip(&self) -> Option<Rect> {
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
                } else {
                    a.next_frame("move_right".to_owned());
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

    pub fn draw(&self, r: &mut Renderer) {
        match self.img {
            Some(ref img) => {
                r.copy(img,
                          self.gen_clip(), /* if self.clip == None {
                                            * self.gen_clip()
                                            * } else {
                                            * self.clip
                                            * } */
                          Some(self.get_rect()))
                    .expect("render failed");
            }
            None => {}
        }

        // debug
        r.set_draw_color(Color::RGB(0, 0, 255));
        r.draw_rect(self.get_rect())
            .expect("fill_rect failed");
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
