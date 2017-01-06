use std::fs::File;
use std::io::{Write, Read};

use sdl2::EventPump;
use sdl2::mouse::MouseButton;

use sdl2::rect::{Rect, Point};

use assets::*;
use entity::Entity;

pub struct Map<'a> {
    pub items: Vec<Entity<'a>>,
    pub img_s: &'a Assets,

    can_place: bool,
    can_remove: bool,
}

impl<'a> Map<'a> {
    pub fn new(img: &'a Assets) -> Map<'a> {
        Map {
            items: vec![],
            img_s: img,
            can_place: true,
            can_remove: true,
        }
    }

    pub fn update(&mut self, event_pump: &EventPump) {
        // placing needs debug
        // add block
        if event_pump.mouse_state().is_mouse_button_pressed(MouseButton::Left) && self.can_place {
            let mut x = event_pump.mouse_state().x();
            let mut y = event_pump.mouse_state().y();

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
            }

            if let None = self.intersect(&Rect::new(x - 16, y - 16, 32, 32)) {
                let t_block = Entity::block(&self.img_s, x, y);
                self.items.push(t_block);
            }

            self.can_place = false;
        } else if !event_pump.mouse_state().is_mouse_button_pressed(MouseButton::Left) {
            self.can_place = true;
        }

        // remove block
        if event_pump.mouse_state().is_mouse_button_pressed(MouseButton::Right) && self.can_remove {
            if let Some(id) =
                self.clicked(event_pump.mouse_state().x(), event_pump.mouse_state().y()) {
                self.items.remove(id);
            }

            self.can_remove = false;
        } else if !event_pump.mouse_state().is_mouse_button_pressed(MouseButton::Right) {
            self.can_remove = true;
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

    pub fn clicked(&self, x: i32, y: i32) -> Option<usize> {

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
                    file.write(format!("{:?}:{:?}\n", m_item.x, m_item.y).into_bytes().as_slice())
                        .unwrap();
                }
            }
        };
    }

    pub fn load_map(&mut self) {
        match File::open("map.data") {
            Err(_) => panic!("failed read map.data"),
            Ok(mut file) => {
                let mut r_str = String::new();

                file.read_to_string(&mut r_str).unwrap();

                for val in r_str.lines() {
                    let pos: Vec<&str> = val.split(':').collect();

                    if pos.len() > 1 {
                        let m_item = Entity::block(self.img_s,
                                                   pos[0].parse::<i32>().unwrap(),
                                                   pos[1].parse::<i32>().unwrap());
                        self.items.push(m_item);
                    }
                }
            }
        };
    }
}
