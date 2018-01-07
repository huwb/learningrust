#[derive(Debug, PartialEq)]
enum MyList {
    Cons(i32, Box<MyList>),
    Nil,
}

impl MyList {
    fn cons(value: i32, next: MyList) -> MyList {
        MyList::Cons(value, Box::new(next))
    }
}

pub fn run() {
    println!("yoyoyo");

    let b = Box::new(5);
    println!("{}", b);

    let l = MyList::cons(7, MyList::cons(3, MyList::cons(5, MyList::Nil)));
    println!("{:?}", l);

    assert_eq!(MyList::cons(7, MyList::Nil), MyList::cons(7, MyList::Nil));
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
