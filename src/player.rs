use Entity;
use ImageS;

pub struct Player<'a> {
    pub ent: Entity<'a>,

    pub left: bool,
    pub right: bool,

    pub is_jumping: bool, // name is invalid rename to is_jumping
    pub is_grounded: bool,
}

impl <'a> Player<'a>{
    pub fn new(img_src: &ImageS , x: i32, y: i32 ) ->  Player {
        Player {
            ent : Entity::player(img_src, x, y),
            left: false,
            right: false,
            is_jumping: false,
            is_grounded: false
        }
    }
}