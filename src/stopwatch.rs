extern crate time;

pub struct Stopwatch {
    start_time: u64,
}

impl Stopwatch {
    pub fn start_new() -> Stopwatch {
        Stopwatch {
            start_time: time::precise_time_ns(),
        }
    }

    pub fn restart(&mut self) {
        *self = Stopwatch::start_new();
    }

    // elapsed time in nanoseconds
    pub fn ns(&self) -> f32 {
        (time::precise_time_ns() - self.start_time) as f32
    }

    // elapsed time in microseconds
    pub fn us(&self) -> f32 {
        (time::precise_time_ns() - self.start_time) as f32 / 1000f32
    }

    // elapsed time in milliseconds
    pub fn ms(&self) -> f32 {
        (time::precise_time_ns() - self.start_time) as f32 / 1000000f32
    }

    // elapsed time in seconds
    pub fn s(&self) -> f32 {
        (time::precise_time_ns() - self.start_time) as f32 / 1000000000f32
    }
}
