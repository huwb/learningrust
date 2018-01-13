#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<isize>>, Rc<List>),
    Nil,
}

use self::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

pub fn run() {
    println!("\nUNSAFE");

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    println!("a before: {:?}", a);
    println!("b before: {:?}", b);
    println!("c before: {:?}", c);

    *value.borrow_mut() += 1000;

    println!("a after: {:?}", a);
    println!("b after: {:?}", b);
    println!("c after: {:?}", c);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_mutable_borrow() {
        // lazy for now - reuse run
        run();
    }

    #[test]
    #[should_panic]
    fn runtime_two_mutable_borrows_panics() {
        let value = Rc::new(RefCell::new(0));
        let _borrow1 = value.borrow_mut();
        let _borrow2 = value.borrow_mut();
    }
}
