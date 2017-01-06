use std::path::Path;
use sdl2::render::{Renderer, Texture};
use sdl2::image::LoadTexture;

use library::Library;

use anim::Anim;

pub struct Assets {
    pub images: ImageS,
    pub animations: AnimsS,
}
pub struct ImageS {
    pub lib: Vec<(String, Texture)>,
}

pub struct AnimsS {
    pub lib: Vec<(String, Anim)>,
}

impl Assets {
    pub fn new() -> Assets {
        Assets {
            images: ImageS::new(),
            animations: AnimsS::new(),
        }
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
