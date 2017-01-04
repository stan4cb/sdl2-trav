use sdl2::rect::Rect;

pub struct Anim {
    key: usize,
    s_key: String,
    frames: Vec<(String, Vec<(Rect)>)>,

    pub r: Rect,
}

impl Anim {
    pub fn new() -> Anim {
        Anim {
            key: 0,
            s_key: String::new(),
            frames: vec![],
            r: Rect::new(0, 0, 0, 0),
        }
    }

    pub fn new_with_r(def: Rect, val: &Vec<(String, Vec<(Rect)>)>) -> Anim {
        Anim {
            key: 0,
            s_key: String::new(),
            frames: val.clone(),
            r: def,
        }
    }

    pub fn next_frame(&mut self, val: String) {
        self.key += 1;
        self.s_key = val;

        for &(ref s_key, ref val) in self.frames.as_slice() {
            if s_key.clone() == self.s_key {
                if self.key >= val.len() {
                    self.key = 0;
                }

                self.r = val[self.key];

                break;
            }
        }

    }

    pub fn g_key(&self) -> usize {
        self.key
    }
}
