extern crate sdl2;

use std::path::Path;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{INIT_PNG, INIT_JPG};

pub mod map;
pub mod anim;
pub mod entity;
pub mod assets;
pub mod player;
pub mod render;
pub mod update;
pub mod events;
pub mod library;
pub mod shurikens;

use assets::Assets;
use entity::Entity;
use player::Player;
use update::Updateble;
use events::EventListener;

//use std::rc::Rc;

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

    let mut world = map::Map::new(&m_assets);

    world.load_map();

    let mut player = player::Player::new(&m_assets, library::SCREEN_WIDTH as i32 / 2, 0);
    // player.shurikens.owner = Some( Rc::from(  &player));
    player.ent.anim_next();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    world.save_map();
                    break 'running;
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

        renderer.set_draw_color(Color::RGB(25, 25, 25));
        renderer.clear();

        render::draw_all(&mut renderer, &[&world, &player.shurikens, &player]);

        renderer.present();
    }
}