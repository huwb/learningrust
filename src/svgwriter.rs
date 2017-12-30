// from http://keepcalmandlearnrust.com/2017/03/polymorphism-in-rust-enum-vs-trait-struct/
use std::fmt;

trait SvgWriter {
    fn write(&self);
}

struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

struct Circle {
    x: u32,
    y: u32,
    radius: u32,
}

impl Circle {
    fn new(x: u32, y: u32, radius: u32) -> Circle {
        Circle { x, y, radius }
    }
}

impl SvgWriter for Circle {
    fn write(&self) {
        println!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\">",
            self.x,
            self.y,
            self.radius
        );
    }
}

struct PolyLine {
    points: Vec<Point>,
}

impl SvgWriter for PolyLine {
    fn write(&self) {
        let myfold = |s: String, p: &Point| -> String {
            if s.len() == 0 {
                format!("{}", p)
            } else {
                format!("{}, {}", s, p)
            }
        };

        let points: String = self.points.iter().fold(String::from(""), myfold);

        println!("<polyline points=\"{}\">", points);
    }
}

fn write_svg(shapes: Vec<Box<SvgWriter>>) {
    println!("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>");
    println!("<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">");

    for shape in shapes {
        shape.write();
    }

    println!("</svg>");
}

fn main() {
    let shapes: Vec<Box<SvgWriter>> = vec![
        Box::new(Circle::new(40, 20, 90)),
        Box::new(PolyLine {
            points: vec![Point::new(1, 2), Point::new(6, 8)],
        }),
    ];

    write_svg(shapes);
}
