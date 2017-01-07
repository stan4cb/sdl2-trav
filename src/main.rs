extern crate sdl2;

use std::path::Path;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{INIT_PNG, INIT_JPG};
use sdl2::rect::Rect;

pub mod map;
pub mod anim;
pub mod entity;
pub mod assets;
pub mod player;
pub mod render;
pub mod update;
pub mod events;
pub mod library;

use map::Map;
use assets::Assets;
use entity::Entity;
use player::Player;
use update::Updateble;
use events::EventListener;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();

    let window = video_subsystem.window("sdl2-trav", library::SCREEN_WIDTH, library::SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer()
        .present_vsync()
        .build()
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut m_assets = Assets::new();

    m_assets.images.load_images(&renderer,
                                &[("player", &Path::new("assets/player.png")),
                                  ("shuriken", &Path::new("assets/shuriken.png")),
                                  ("block", &Path::new("assets/block.png"))]);

    m_assets.animations.load_animations(&[("player", &Path::new("assets/player.anim")),
                                          ("shuriken", &Path::new("assets/shuriken.anim"))]);

    let mut world = Map::new(&m_assets);

    world.load_map();

    let mut player = Player::new(&m_assets, library::SCREEN_WIDTH as i32 / 2, 0);
    player.ent.anim_next();

    let mut shir: Vec<Entity> = vec![];

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    world.save_map();
                    break 'running;
                }
                Event::KeyUp { keycode: Some(Keycode::Z), .. } => {
                    if player.ent.dir != 0 {
                        let x = player.ent.x + (player.ent.dir as i32 * (16 + 8 + 1));

                        if let Some(_) = world.intersect(&Rect::new(x, player.ent.y, 16, 16)) {
                        } else {
                            let t_shir =
                                Entity::shuriken(&m_assets, x, player.ent.y, player.ent.dir);

                            shir.push(t_shir);
                        }
                    }
                }
                Event::KeyUp { keycode: Some(key), .. } => player.key_up(key),
                Event::KeyDown { keycode: Some(key), .. } => player.key_down(key),
                Event::MouseButtonUp { mouse_btn, x, y, .. } => world.mouse_up(mouse_btn, x, y),
                Event::MouseButtonDown { mouse_btn, x, y, .. } => world.mouse_down(mouse_btn, x, y),
                _ => {}
            }
        }

        player.update(); // eh
        player.update_with_world(&world);

        let mut remove: Vec<usize> = vec![];
        for i in 0..shir.len() {
            let ref mut val = shir[i];
            if !val.is_offscreen() {
                let s_speed = (val.dir as i32) * 7;

                if let Some(ret) = world.intersect(&val.trans_rect(s_speed, 0)) {
                    if let Some(_) = player.ent.get_rect().intersection(val.get_rect()) {
                        remove.push(i as usize);
                    }
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
            shir.remove(remove[remove.len() - i - 1]);
        }

        renderer.set_draw_color(Color::RGB(25, 25, 25));
        renderer.clear();

        render::draw_all(&mut renderer, &[&world, &player, &shir]);

        renderer.present();
    }
}