use std::ops::Deref;

#[derive(Debug, PartialEq)]
enum MyList {
    Cons(i32, Box<MyList>),
    Nil,
}

impl MyList {
    fn new() -> MyList {
        MyList::Nil
    }

    fn cons(value: i32, next: MyList) -> MyList {
        MyList::Cons(value, Box::new(next))
    }

    /// Add value to end of list
    fn add(list: MyList, value: i32) -> MyList {
        match list {
            MyList::Cons(val, boxed) => MyList::Cons(val, Box::new(MyList::add(*boxed, value))),
            MyList::Nil => MyList::Cons(value, Box::new(MyList::Nil)),
        }
    }

    /// Push value onto end of list
    fn push(&mut self, value: i32) {
        match *self {
            MyList::Cons(_, ref mut boxed) => boxed.push(value),
            MyList::Nil => *self = MyList::Cons(value, Box::new(MyList::Nil)),
        };
    }
}

pub fn run() {
    println!("yoyoyo");

    let b = Box::new(5);
    println!("{}", b);

    let mut l = MyList::new();
    l = MyList::cons(29, l);
    l = MyList::add(l, 33);
    l.push(11);
    println!("{:?}", l);

    let x = 5;
    let y = &x;
    assert_eq!(x, 5);
    assert_eq!(*y, x);

    let y = Box::new(x);
    assert_eq!(Box::new(x), y);
    assert_eq!(x, *y);

    let y = MyBox::new(x);
    assert_eq!(x, *y);

    let daname = MyBox::new(String::from("Yoheyo"));
    hello(&(*daname)[..]); // manually convert from MyBox<String> to &str
    hello(&daname); // deref coercion
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Heyooooo {}", name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mylist_constructors() {
        let l = MyList::cons(7, MyList::cons(3, MyList::cons(5, MyList::Nil)));
        assert_eq!(
            l,
            MyList::Cons(
                7,
                Box::new(MyList::Cons(
                    3,
                    Box::new(MyList::Cons(5, Box::new(MyList::Nil)))
                ))
            )
        );
    }

    #[test]
    fn mylist_add() {
        let mut l = MyList::new();
        l = MyList::add(l, 3);
        l = MyList::add(l, 7);
        l = MyList::add(l, 5);
        assert_eq!(
            l,
            MyList::Cons(
                3,
                Box::new(MyList::Cons(
                    7,
                    Box::new(MyList::Cons(5, Box::new(MyList::Nil)))
                ))
            )
        );
    }

    #[test]
    fn mylist_push() {
        let mut l = MyList::new();
        l.push(9);
        l.push(6);
        l.push(8);
        assert_eq!(
            l,
            MyList::Cons(
                9,
                Box::new(MyList::Cons(
                    6,
                    Box::new(MyList::Cons(8, Box::new(MyList::Nil)))
                ))
            )
        );
    }
}

// impl PartialEq for MyList {
//     fn eq(&self, other: &MyList) -> bool {
//         match (self, other) {
//             (&MyList::Nil, &MyList::Nil) => true,
//             (&MyList::Cons(v1, ref boxed1), &MyList::Cons(v2, ref boxed2)) => {
//                 (v1 == v2) && (*boxed1 == *boxed2)
//             }
//             _ => false,
//         }
//     }
// }
