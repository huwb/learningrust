use std::f64;
use std::ops::Add;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// This will kill Copy behaviour because Rust likes to keep copies ultra-simple
// (i.e. memcpy), and if a struct has a destructor, or if any of its members have
// a destructor, the copy is no longer simple.
// https://www.reddit.com/r/rust/comments/5omcoy/new_to_rust_confused_by_copy_vs_move_and/
// impl Drop for Point {
//     fn drop(&mut self) {
//         println!("Droppity-drop: {:?}", self);
//     }
// }

pub fn run() {
    println!("POINTS");

    let p1 = Point::origin();
    let p2 = Point::new(1.0, 2.0);
    let p3 = p1 + p2;

    println!("Answer: {:?}", p3);
    println!("Length: {}", p3.length());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_points() {
        let p0 = Point::origin();
        let p1 = Point::new(3.0, 4.0);
        let p2 = Point::new(5.0, 6.0);

        let p3 = p0 + p1 + p2;

        assert_eq!(p3.x, p1.x + p2.x);
        assert_eq!(p3.y, p1.y + p2.y);
    }

    #[test]
    fn copy_constructor() {
        let p0 = Point::origin();
        let p1 = p0;
        // use p0 after assignment - check that it has been copied
        assert_eq!(p0.length(), 0.0);
        assert_eq!(p1.length(), 0.0);
    }
}
