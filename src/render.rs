use sdl2::pixels::Color;
use sdl2::render::Renderer;

use camera::Camera;

pub fn draw_all(cam: &Camera,target: &mut Renderer, r: &[&Renderable]) {
    target.set_draw_color(Color::RGB(26, 107, 160));
    target.clear();

    target.set_scale(cam.scale, cam.scale).unwrap();

    for ref i in r {
        i.draw(target);
    }
    
    target.present();
}

pub trait Renderable {
    fn draw(&self, _: &mut Renderer);
}

impl<'a, T: Renderable> Renderable for Vec<T> {
    fn draw(&self, r: &mut Renderer) {
        for ref i in self {
            i.draw(r);
        }
    }
}