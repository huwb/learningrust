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
