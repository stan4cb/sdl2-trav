use sdl2::render::Texture;

use assets::*;
use anim::Anim;

pub const SCREEN_WIDTH: u32 = 1024;
pub const SCREEN_HEIGHT: u32 = 768;

pub trait Library<'a, T, OptionT> {
    fn add(&mut self, _: &str, _: T);
    fn get(self: &'a Self, _: &str) -> OptionT;
}

impl<'a> Library<'a, Texture, Option<&'a Texture>> for ImageS {
    fn add(&mut self, key: &str, item: Texture) {
        self.lib.push((key.to_owned(), item))
    }

    fn get(self: &'a Self, val: &str) -> Option<&'a Texture> {
        let s_comp = String::from(val);

        for i in 0..self.lib.len() {
            let ref value: (String, Texture) = self.lib.as_slice()[i];
            if value.0.to_owned() == s_comp {
                return Some(&value.1);
            }
        }

        return None;
    }
}

impl<'a> Library<'a, Anim, Option<Anim>> for AnimsS {
    fn add(&mut self, key: &str, item: Anim) {
        self.lib.push((key.to_owned(), item))
    }

    fn get(self: &'a Self, val: &str) -> Option<Anim> {
        let s_comp = String::from(val);

        for i in 0..self.lib.len() {
            let ref value: (String, Anim) = self.lib.as_slice()[i];
            if value.0.to_owned() == s_comp {
                return Some(value.1.clone());
            }
        }

        return None;
    }
}
