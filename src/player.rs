use Entity;
use Assets;

pub struct Player<'a> {
    pub ent: Entity<'a>,

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
            ent: Entity::player(img_src, x, y),
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
