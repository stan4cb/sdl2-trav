use std::fs::File;
use std::io::{Write, Read};

use sdl2::rect::{Rect, Point};

use assets::*;
use entity::Entity;

use events::EventListener;
use sdl2::mouse::MouseButton;

use render::Renderable;
use sdl2::render::Renderer;

pub struct Map<'a> {
    pub items: Vec<Entity<'a>>,
    pub img_s: &'a Assets,

    pub can_place: bool,
    pub can_remove: bool,
    pub bits: i32,
}

impl<'a> Map<'a> {
    pub fn new(img: &'a Assets, b: i32) -> Map<'a> {
        Map {
            items: vec![],
            img_s: img,
            can_place: true,
            can_remove: true,
            bits: b,
        }
    }

    pub fn intersect(&self, r: &Rect) -> Option<Rect> {
        for ent in self.items.as_slice() {
            if let Some(val) = r.intersection(ent.get_rect()) {
                return Some(val);
            }
        }

        None
    }

    pub fn contains(&self, x: i32, y: i32) -> Option<usize> {
        for i in 0..self.items.len() {
            let ref item = self.items[i];

            if item.get_rect()
                .contains(Point::new(x, y)) {
                return Some(i);
            }
        }

        None
    }

    pub fn save_map(&self) {
        match File::create("map.data") {
            Err(_) => panic!("failed create map.data"),
            Ok(mut file) => {
                for m_item in self.items.as_slice() {
                    file.write(m_item.to_string().into_bytes().as_slice())
                        .unwrap();
                }
            }
        };
    }

    pub fn load_map(&mut self) {
        match File::open("map.data") {
            Err(_) => {}
            Ok(mut file) => {
                let mut r_str = String::new();

                file.read_to_string(&mut r_str).unwrap();

                for val in r_str.lines() {
                    let pos: Vec<&str> = val.split(':').collect();

                    if pos.len() > 1 {
                        let mut b_type = String::from("block");

                        if pos.len() > 2 {
                            b_type = String::from(pos[2]);
                        }

                        let m_item = Entity::map_item(self.img_s,
                                                      b_type,
                                                      pos[0].parse::<i32>().unwrap(),
                                                      pos[1].parse::<i32>().unwrap());

                        self.items.push(m_item);
                    }
                }
            }
        };
    }
}

impl<'a> Renderable for Map<'a> {
    fn draw(&self, r: &mut Renderer) {
        for ref i in self.items.as_slice() {
            i.draw(r);
        }
    }
}

impl<'a> EventListener for Map<'a> {
    fn mouse_up(&mut self, btn: MouseButton, _: i32, _: i32) {
        match btn {
            MouseButton::Left => self.can_place = true,
            MouseButton::Right => self.can_remove = true,
            _ => {}
        }
    }

    fn mouse_down(&mut self, btn: MouseButton, x: i32, y: i32) {
        let div = self.bits;
        let half = self.bits / 2;
        match btn {
            MouseButton::Left => {
                if self.can_place {
                    let mut x = x;
                    let mut y = y;

                    let re = x % div;
                    if re != 0 {
                        if re > half {
                            x = (x / div + 1) * div;
                        } else {
                            x = (x / div) * div;
                        }
                    }
                    let re = y % div;
                    if re != 0 {
                        if re > half {
                            y = (y / div + 1) * div;
                        } else {
                            y = (y / div) * div;
                        }
                    }

                    if let None = self.intersect(&Rect::new(x - half,
                                                            y - half,
                                                            self.bits as u32,
                                                            self.bits as u32)) {
                        let t_block = Entity::map_item(&self.img_s, "OB_Blue".to_owned(), x, y);
                        self.items.push(t_block);
                    }
                }
            }
            MouseButton::Right => {
                if self.can_remove {
                    if let Some(id) = self.contains(x, y) {
                        self.items.remove(id);
                    }

                    self.can_remove = false;
                }
            }
            _ => {}
        }
    }
}
