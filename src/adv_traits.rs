pub fn run() {
    println!("in run");

    let mut m = MyStruct { counter: 3 };

    println!("{}", m.next().unwrap());
    println!("{}", m.next().unwrap());
    println!("{}", m.next().unwrap());
}

struct MyStruct {
    counter: isize,
}

pub trait MyIter {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl MyIter for MyStruct {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        self.counter -= 1;
        if self.counter > 0 {
            Some(self.counter)
        } else {
            None
        }
    }
}

trait GGraph<Node, Edge> {
    fn display();
}

trait AGraph {
    type Node;
    type Edge;

    fn display();
}

fn distance<G: AGraph>(g: G, n0: G::Node, n1: G::Node) -> f32 {
    0.0
}

fn dist<N, E, G: GGraph<N, E>>(g: G, n0: N, n1: N) -> f32 {
    0.0
}
