extern crate time;

use std::ops::Div;

pub trait ConvertToFloat {
    fn convert(x: u64) -> Self;
}
impl ConvertToFloat for f32 {
    #[inline]
    fn convert(x: u64) -> Self {
        x as f32
    }
}
impl ConvertToFloat for f64 {
    #[inline]
    fn convert(x: u64) -> Self {
        x as f64
    }
}

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
    pub fn elapsed_ns<T: ConvertToFloat>(&self) -> T {
        T::convert(time::precise_time_ns() - self.start_time)
    }

    // elapsed time in microseconds
    pub fn elapsed_us<T: ConvertToFloat + Div>(&self) -> T::Output {
        T::convert(time::precise_time_ns() - self.start_time) / T::convert(1000)
    }

    // elapsed time in milliseconds
    pub fn elapsed_ms<T: ConvertToFloat + Div>(&self) -> T::Output {
        T::convert(time::precise_time_ns() - self.start_time) / T::convert(1000000)
    }

    // elapsed time in seconds
    pub fn elapsed_s<T: ConvertToFloat + Div>(&self) -> T::Output {
        T::convert(time::precise_time_ns() - self.start_time) / T::convert(1000000000)
    }
}
