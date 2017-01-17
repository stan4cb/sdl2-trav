use sdl2::pixels::Color;
use sdl2::render::Renderer;

pub fn draw_all(target: &mut Renderer, r: &[&Renderable]) {
    target.set_draw_color(Color::RGB(26, 107, 160));
    target.clear();

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