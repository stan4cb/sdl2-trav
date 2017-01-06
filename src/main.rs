extern crate sdl2;

use std::path::Path;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{INIT_PNG, INIT_JPG};
use sdl2::rect::Rect;

pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;

pub mod map;
pub mod entity;
pub mod assets;
pub mod player;
pub mod anim;
pub mod render;

use map::Map;
use assets::ImageS;
use entity::Entity;
use player::Player;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();

    let window = video_subsystem.window("sdl2-trav", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer()
        .present_vsync()
        .build()
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut img_assets = ImageS::new();
    img_assets.load_images(&renderer,
                           &[("player", &Path::new("assets/player.png")),
                             ("shuriken", &Path::new("assets/shuriken.png")),
                             ("block", &Path::new("assets/block.png"))]);

    let mut world = Map::new(&img_assets);

    world.load_map();

    let mut player = Player::new(&img_assets, SCREEN_WIDTH as i32 / 2, 0);
    player.ent.anim_next();

    let mut jump_buffer = 0_i32; // move it to player

    let speed = 6;
    let gravity = 5;
    let jump_speed = 8;

    let mut shir: Vec<Entity> = vec![];

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    world.save_map();
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    if !player.is_jumping && player.is_grounded {
                        player.is_jumping = true;
                        jump_buffer = 32 * 3;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => player.left = true,
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => player.right = true,
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => player.left = false,
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => player.right = false,
                Event::KeyUp { keycode: Some(Keycode::Z), .. } => {
                    if player.ent.dir != 0 {
                        let x = player.ent.x + (player.ent.dir as i32 * (16 + 8 + 1));

                        if let Some(_) = world.intersect(&Rect::new(x, player.ent.y, 16, 16)) {
                        } else {
                            let t_shir =
                                Entity::shuriken(&img_assets, x, player.ent.y, player.ent.dir);

                            shir.push(t_shir);
                        }
                    }
                }
                _ => {}
            }
        }

        world.update(&event_pump);

        // face direction, allows throwing
        player.ent.dir = if player.left {
            -1
        } else if player.right {
            1
        } else {
            player.ent.dir
        };

        // border left
        if player.left && player.ent.x > 16 {
            if let Some(val) = world.intersect(&player.ent.trans_rect(-speed, 0)) {
                player.ent.x = val.right() + 16;
            } else {
                player.ent.x -= speed;
                player.ent.anim_next();
            }
        } else if player.left {
            player.ent.x = 16;
        }

        // border right
        if player.right && player.ent.x < 800 - 16 {
            if let Some(val) = world.intersect(&player.ent.trans_rect(speed, 0)) {
                player.ent.x = val.left() - 16;
            } else {
                player.ent.x += speed;
                player.ent.anim_next();
            }
        } else if player.right {
            player.ent.x = 800 - 16;
        }

        if !player.left && !player.right {
            // player.ent.dir = 0;
            // player.ent.anim_next();
        }

        if player.ent.y < 600 - 32 && !player.is_jumping {
            if let Some(val) = world.intersect(&player.ent.trans_rect(0, gravity)) {
                player.ent.y = val.top() - 32;
                player.is_grounded = true;
            } else {
                player.ent.y += gravity;
                player.is_grounded = false;
            }

        } else if jump_buffer > 0 {
            jump_buffer -= jump_speed;

            if let Some(val) = world.intersect(&player.ent.trans_rect(0, -jump_speed)) {
                player.ent.y = val.bottom() + 32;
                jump_buffer = 0;
                player.is_jumping = false;
            } else {
                player.ent.y -= jump_speed;
            }


        } else {
            player.is_jumping = false;
            player.is_grounded = true; // nes?
        }

        let mut remove: Vec<usize> = vec![];
        for i in 0..shir.len() {
            let ref mut val = shir[i];
            // val.x > -8 && val.x < 800 + 8
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