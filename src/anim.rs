use sdl2::rect::Rect;

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Anim {
    key: usize,
    s_key: String,
    frames: Vec<(String, Vec<(Rect)>)>,

    pub r: Rect,
}

impl Anim {
    pub fn new_empty() -> Anim {
        Anim {
            key: 0,
            s_key: String::new(),
            frames: vec![],
            r: Rect::new(0, 0, 0, 0),
        }
    }

    pub fn new(def: Rect, val: &Vec<(String, Vec<(Rect)>)>) -> Anim {
        Anim {
            key: 0,
            s_key: String::new(),
            frames: val.clone(),
            r: def,
        }
    }

    pub fn load_from_file(p: &Path) -> Anim {
        let mut t_anim = Anim::new_empty();

        match File::open(p) {
            Err(_) => panic!("failed to read {:?}", p),
            Ok(mut file) => {
                let mut r_str = String::new();

                file.read_to_string(&mut r_str).unwrap();

                let mut frame_name = String::new();
                let mut r_pos: Vec<Rect> = vec![];

                for val in r_str.lines() {
                    if val.starts_with('.') {
                        if r_pos.len() > 0 {
                            t_anim.frames.push((frame_name, r_pos.clone()));
                            r_pos.clear();
                        }

                        let mut ow = val.to_owned();
                        ow.remove(0);
                        frame_name = ow;
                    } else {
                        let pos: Vec<&str> = val.split(':').collect();

                        if pos.len() > 3 {
                            let x = pos[0].parse::<i32>().unwrap();
                            let y = pos[1].parse::<i32>().unwrap();
                            let w = pos[2].parse::<i32>().unwrap();
                            let h = pos[3].parse::<i32>().unwrap();
                            r_pos.push(Rect::new(x, y, w as u32, h as u32));
                        }
                    }
                }

                t_anim.frames.push((frame_name, r_pos.clone()));
            }
        };

        return t_anim;
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
