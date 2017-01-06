use std::path::Path;
use sdl2::render::{Renderer, Texture};
use sdl2::image::LoadTexture;

use anim::Anim;

pub struct Assets {
    pub images: ImageS,
    pub animations: AnimsS,
}

impl Assets {
    pub fn new() -> Assets {
        Assets {
            images: ImageS::new(),
            animations: AnimsS::new(),
        }
    }
}

pub struct ImageS {
    lib: Vec<(String, Texture)>,
}

pub struct AnimsS {
    lib: Vec<(String, Anim)>,
}

pub trait Library<'a, T, OptionT> {
    fn add(&mut self, _: &str, _: T);
    fn get_ref(self: &'a Self, _: &str) -> OptionT;
}

impl<'a> Library<'a, Texture, Option<&'a Texture>> for ImageS {
    fn add(&mut self, key: &str, item: Texture) {
        self.lib.push((key.to_owned(), item))
    }

    fn get_ref(self: &'a Self, val: &str) -> Option<&'a Texture> {
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

    fn get_ref(self: &'a Self, val: &str) -> Option<Anim> {
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


impl ImageS {
    pub fn new() -> ImageS {
        ImageS { lib: vec![] }
    }

    pub fn load_images(&mut self, r: &Renderer, list: &[(&str, &Path)]) {
        for i in list {
            self.add(i.0, r.load_texture(i.1).unwrap());
        }
    }
}

impl AnimsS {
    pub fn new() -> AnimsS {
        AnimsS { lib: vec![] }
    }

    pub fn load_animations(&mut self,list: &[(&str, &Path)]) {
        for i in list {
            self.add(  i.0, Anim::load_from_file(i.1) );
        }
    }
}
