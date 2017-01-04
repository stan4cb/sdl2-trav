use std::path::Path;
use sdl2::render::{Renderer, Texture};
use sdl2::image::LoadTexture;

pub struct ImageS {
    images: Vec<(String, Texture)>,
}

impl ImageS {
    pub fn new() -> ImageS {
        ImageS { images: vec![] }
    }

    pub fn add_image(&mut self, name: &str, img: Texture) {
        self.images.push((String::from(name), img))
    }

    pub fn load_images(&mut self, r: &Renderer, list: &[(&str, &Path)]) {
        for i in list {
            self.add_image(i.0, r.load_texture(i.1).unwrap());
        }
    }

    pub fn get_image(&self, val: &str) -> Option<&Texture> {
        let s_comp = String::from(val);

        for i in 0..self.images.len() {
            let (ref s, ref i): (String, Texture) = self.images.as_slice()[i];
            if s.to_owned() == s_comp {
                return Some(i);
            }
        }

        return None;
    }
}
