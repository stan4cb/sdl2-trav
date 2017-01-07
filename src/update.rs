use Player;
use map::Map;

pub trait Updateble {
    fn update(&mut self);
    fn update_with_world(&mut self, world: &Map);
}

impl<'a> Updateble for Player<'a> {
    fn update(&mut self) {
        // fix direction
        self.ent.dir = if self.left {
            -1
        } else if self.right {
            1
        } else {
            self.ent.dir
        };

    }

    fn update_with_world(&mut self, world: &Map) {
        // move left
        if self.left && self.ent.x > 16 {
            if let Some(val) = world.intersect(&self.ent.trans_rect(-self.speed, 0)) {
                self.ent.x = val.right() + 16;
            } else {
                self.ent.x -= self.speed;
                self.ent.anim_next();
            }
        } else if self.left {
            self.ent.x = 16;
        }

        // move right
        if self.right && self.ent.x < 800 - 16 {
            if let Some(val) = world.intersect(&self.ent.trans_rect(self.speed, 0)) {
                self.ent.x = val.left() - 16;
            } else {
                self.ent.x += self.speed;
                self.ent.anim_next();
            }
        } else if self.right {
            self.ent.x = 800 - 16;
        }

        if self.ent.y < 600 - 32 && !self.is_jumping {
            if let Some(val) = world.intersect(&self.ent.trans_rect(0, self.gravity)) {
                self.ent.y = val.top() - 32;
                self.is_grounded = true;
            } else {
                self.ent.y += self.gravity;
                self.is_grounded = false;
            }

        } else if self.jump_buffer > 0 {
            self.jump_buffer -= self.jump_speed;

            if let Some(val) = world.intersect(&self.ent.trans_rect(0, -self.jump_speed)) {
                self.ent.y = val.bottom() + 32;
                self.jump_buffer = 0;
                self.is_jumping = false;
            } else {
                self.ent.y -= self.jump_speed;
            }
        } else {
            self.is_jumping = false;
            self.is_grounded = true; // nes?
        }

        self.shurikens.update_with_world(world);
    }
}
