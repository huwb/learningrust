
//use std::io;
use std::thread;
use std::time;
use std::f64;
use std::ops::Add;

#[derive(Clone,Copy)]
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
        f64::sqrt( self.x*self.x + self.y*self.y )
    }
}

impl Add for Point {
    type Output = Point;

    fn add( self, other: Point ) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

#[test]
fn add_points() -> () {
    let p1 = Point::new(3.0, 4.0);
    let p2 = Point::new(5.0, 6.0);

    let p3 = p1 + p2;

    assert_eq!(p3.x, p1.x + p2.x);
    assert_eq!(p3.y, p1.y + p2.y);
}

fn main()
{
    let p1 = Point::origin();
    let p2 = Point { x: 1.0, y: 2.0 };
    let mut p3 = Point::new( 3.0, 4.0 );

    p3 = p1 + p2;

    println!("Answer: {}", p3.length() );
}
