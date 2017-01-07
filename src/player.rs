use Entity;
use Assets;
use shurikens::Shurukens;

pub struct Player<'a> {
    pub asset_base: &'a Assets,
    pub ent: Entity<'a>,
    pub shurikens: Shurukens<'a>,

    pub left: bool,
    pub right: bool,

    pub is_jumping: bool, // name is invalid rename to is_jumping
    pub is_grounded: bool,

    pub speed: i32,

    pub jump_buffer: i32,
    pub jump_speed: i32,
    pub gravity: i32,
}

impl<'a> Player<'a> {
    pub fn new(img_src: &Assets, x: i32, y: i32) -> Player {
        Player {
            asset_base: img_src,
            ent: Entity::player(img_src, x, y),
            shurikens: Shurukens::new(),
            left: false,
            right: false,
            is_jumping: false,
            is_grounded: false,
            speed: 6,
            jump_buffer: 0,
            jump_speed: 5,
            gravity: 8,
        }
    }
}
