use Entity;
use map::Map;
use update::Updateble;
use render::Renderable;
use assets::Assets;
use player::Player;

use sdl2::render::Renderer;
use std::rc::Rc;

pub struct Shurukens<'a> {
    pub items: Vec<Entity<'a>>,
    pub owner: Option<Rc<&'a Player<'a>>>,
    max: usize,
}

impl<'a> Shurukens<'a> {
    pub fn new() -> Shurukens<'a> {
        Shurukens {
            owner: None,
            items: vec![],
            max: 5,
        }
    }

    pub fn throw(&mut self, assets: &'a Assets, dir: i8, x: i32, y: i32) {
        if self.items.len() < self.max {
            let t_shir = Entity::shuriken(assets, x, y, dir);

            self.items.push(t_shir);
        }
    }
}

impl<'a> Renderable for Shurukens<'a> {
    fn draw(&self, r: &mut Renderer) {
        for ref i in self.items.as_slice() {
            i.draw(r);
        }
    }
}

impl<'a> Updateble for Shurukens<'a> {
    fn update(&mut self) {}

    fn update_with_world(&mut self, world: &Map) {
        let mut remove: Vec<usize> = vec![];
        for i in 0..self.items.len() {
            let ref mut val = self.items[i];
            if !val.is_offscreen() {
                let s_speed = (val.dir as i32) * 7;

                if let Some(ret) = world.intersect(&val.trans_rect(s_speed, 0)) {
                    // find a way to access Player

                    // if let Some(_) = player.ent.get_rect().intersection(val.get_rect()) {
                    // remove.push(i as usize);
                    // }

                    // should stick to the block?
                    if true {
                        // ret.height() > 6
                        val.x = if val.dir == 1 {
                            ret.left() - 8
                        } else {
                            ret.right() + 8
                        };
                    } else {
                        val.x += s_speed;
                        val.anim_next();
                    }
                } else {
                    val.x += s_speed;
                    val.anim_next();
                }
            } else {
                remove.push(i as usize);
            }
        }

        for i in 0..remove.len() {
            self.items.remove(remove[remove.len() - i - 1]);
        }
    }
}