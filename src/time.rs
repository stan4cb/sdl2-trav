use std::time::Instant;

pub struct Timer {
    last_time: Instant,
    delta_time: u32,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            last_time: Instant::now(),
            delta_time: 0_u32,
        }
    }

    pub fn update(&mut self) {
        self.delta_time = self.last_time.elapsed().subsec_nanos();
        self.last_time = Instant::now();
    }
}