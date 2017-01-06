use sdl2::render::{Renderer};
use sdl2::pixels::Color;

use player::Player;
use entity::Entity;
use map::Map;

pub fn draw_all(target: &mut Renderer, r: &[&Renderable]) {
    for ref i in r {
        i.draw(target);
    }
}

pub trait Renderable {
    fn draw(&self, _: &mut Renderer);
}

impl <'a> Renderable for Entity<'a> {
    fn draw(&self, r: &mut Renderer) {
        match self.img {
            Some(ref img) => {
                r.copy(img,
                          self.anim_frame(),
                          Some(self.get_rect()))
                    .expect("render failed");
            },
            None => {}
        }

        r.set_draw_color(Color::RGB(0, 0, 255));
        r.draw_rect(self.get_rect())
            .expect("fill_rect failed");
    }
}

impl <'a> Renderable for Player<'a> {
    fn draw(&self,  r: &mut Renderer) {
        self.ent.draw(r);
    }
}

impl <'a, T: Renderable> Renderable for Vec<T> {
    fn draw(&self,  r: &mut Renderer) {
        for ref i in self {
            i.draw(r);
        }
    }
}

impl <'a> Renderable for Map<'a> {
    fn draw(&self, r: &mut Renderer) {
        for ref i in self.items.as_slice() {
            i.draw(r);
        }
    }
}

    