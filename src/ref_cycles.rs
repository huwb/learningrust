#[cfg(test)]
mod tests {

    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    #[derive(Debug)]
    struct Node {
        value: isize,
        parent: RefCell<Weak<Node>>,
        children: Vec<Rc<Node>>,
    }

    #[test]
    fn check_weak_and_strong_counts() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: vec![],
        });

        assert_eq!(Rc::strong_count(&leaf), 1);
        assert_eq!(Rc::weak_count(&leaf), 0);

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: vec![Rc::clone(&leaf)],
            });
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            assert_eq!(Rc::strong_count(&leaf), 2);
            assert_eq!(Rc::weak_count(&leaf), 0);

            assert_eq!(Rc::strong_count(&branch), 1);
            assert_eq!(Rc::weak_count(&branch), 1);
        }

        assert_eq!(Rc::strong_count(&leaf), 1);
        assert_eq!(Rc::weak_count(&leaf), 0);
    }
}
