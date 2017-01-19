extern crate sdl2;

use std::path::Path;

use sdl2::event::Event;
use sdl2::image::{INIT_PNG, INIT_JPG};

pub mod map;
pub mod time;
pub mod anim;
pub mod entity;
pub mod assets;
pub mod player;
pub mod render;
pub mod update;
pub mod events;
pub mod library;
pub mod shurikens;
pub mod camera;

use assets::Assets;
use events::*;
use entity::Entity;
use library::Library;

use camera::*;

pub fn main() {
    let sdl_context = sdl2::init().expect("sdl2 init");
    let video_subsystem = sdl_context.video().expect("video init");

    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).expect("sdl2_image init");

    let window = video_subsystem.window("sdl2-trav", library::SCREEN_WIDTH, library::SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .borderless()
        .build()
        .expect("window init");
        
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_version(3, 2);
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(4);

    let mut renderer = window.renderer()
        .present_vsync()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut m_assets = Assets::new();

    m_assets.images.load_images(&renderer,
                                &[("BG", &Path::new("assets/bg.png")),
                                  ("Player", &Path::new("assets/player.png")),
                                  ("Shuriken", &Path::new("assets/shuriken.png")),
                                  ("OB_Blue", &Path::new("assets/blue.png")),
                                  ("Block", &Path::new("assets/block.png"))]);

    m_assets.animations.load_animations(&[("Player", &Path::new("assets/player.anim")),
                                          ("Shuriken", &Path::new("assets/shuriken.anim"))]);

    let mut world = map::Map::new(&m_assets, 16);

    world.load_map();
    let bg = Entity::place_holder(entity::EType::Block,
                                  m_assets.images.get("_"),
                                  1280 / 2,
                                  720 / 2,
                                  1280,
                                  720);

    // let mut player = player::Player::new(&m_assets, library::SCREEN_WIDTH as i32 / 2, 0);

    let mut cam = Camera::new();
    let mut ev = Listener::new();

    ev.listeners.push(&mut cam);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    world.save_map();
                    break 'running;
                }
                Event::KeyUp { keycode: Some(key), .. } => ev.key_up(key),
                Event::KeyDown { keycode: Some(key), .. } => ev.key_down(key),
                Event::MouseButtonUp { mouse_btn, x, y, .. } => ev.mouse_up(mouse_btn, x, y),
                Event::MouseButtonDown { mouse_btn, x, y, .. } => ev.mouse_down(mouse_btn, x, y),
                _ => {}
            }
        }

        render::draw_all(&cam, &mut renderer, &[&bg, &world]);
    }
}
