use sdl2::rect::Rect;

use assets::*;
use library::*;
use anim::Anim;

use render::Renderable;
use sdl2::pixels::Color;
use sdl2::render::{Texture, Renderer};

#[derive(PartialEq, Clone, Debug)]
pub enum EType {
    Player,
    Shuriken,
    Block,
    OB_Blue,
}

impl EType {
    pub fn from_string(s: &str) -> Option<EType> {
        match s {
            "Block" => Some(EType::Block),
            "Player" => Some(EType::Player),
            "Shuriken" => Some(EType::Shuriken),
            "OB_Blue" => Some(EType::OB_Blue),

            _ => None,
        }
    }
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
    pub fn to_string(&self) -> String {
        format!("{:?}:{:?}:{:?}\n", self.x, self.y, self.m_type)
    }

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
            img: assets.images.get("Player"),
            anim: assets.animations.get("Player"),
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
            img: assets.images.get("Shuriken"),
            anim: assets.animations.get("Shuriken"),
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
            img: assets.images.get("Block"),
            anim: None,
        }
    }

    pub fn map_item(_: &Assets, nm: String, x: i32, y: i32) -> Entity {
        Entity {
            m_type: EType::from_string(nm.as_ref()).unwrap(),
            x: x,
            y: y,
            w: 12,
            h: 12,
            dir: 0,
            img: None, // assets.images.get(nm.as_ref()),
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

impl<'a> Renderable for Entity<'a> {
    fn draw(&self, r: &mut Renderer) {
        match self.img {
            Some(ref img) => {
                r.copy(img, self.anim_frame(), Some(self.get_rect()))
                    .expect("render failed");
            }
            None => {
                // r.set_draw_color(Color::RGB(0,255,0));
            }
        }

        r.set_draw_color(Color::RGB(255, 0, 0));
        r.draw_rect(self.get_rect())
            .expect("fill_rect failed");
    }
}