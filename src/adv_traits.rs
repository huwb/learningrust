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

impl Iterator for MyStruct {
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
