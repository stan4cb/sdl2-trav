pub struct Anim {
    key: u32,
    frame_max: u32,
}

impl Anim {
    pub fn new(f_max: u32) -> Anim {
        Anim {
            key: 0,
            frame_max: f_max,
        }
    }

    pub fn next_frame(&mut self) {
        self.key += 1;

        if self.key == self.frame_max {
            self.key = 0;
        }
    } 

    pub fn g_key(&self) -> u32 {
        self.key
    }
}