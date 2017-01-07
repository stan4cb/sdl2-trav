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

    pub can_place: bool,
    pub can_remove: bool,
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
