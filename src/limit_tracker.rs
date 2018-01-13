struct PrintlnMessenger {}

impl Messenger for PrintlnMessenger {
    fn send(&self, msg: &str) {
        println!("Received a message! {}", msg);
    }
}

pub fn run() {
    println!("\nUNSAFE");

    let plm = PrintlnMessenger {};
    let mut lt = LimitTracker::new(&plm, 100);

    lt.set_value(0);
    lt.set_value(50);
    lt.set_value(70);
    lt.set_value(75);
    lt.set_value(100);
    lt.set_value(110);
}

pub trait Messenger {
    fn send(&self, &str);
}

struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    fn set_value(&mut self, value: usize) {
        // should always increase
        assert!(value > self.value);

        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("You have used up the quota!");
        } else if percentage_of_max >= 0.7 {
            self.messenger
                .send("You have used up over 70% of the quota");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    fn run_test(test_value: usize, expected_messages: usize) {
        let mock_messenger = MockMessenger::new();
        let mut lt = LimitTracker::new(&mock_messenger, 100);

        assert_eq!(mock_messenger.sent_messages.borrow_mut().len(), 0);

        lt.set_value(test_value);

        assert_eq!(
            mock_messenger.sent_messages.borrow_mut().len(),
            expected_messages
        );
    }

    #[test]
    fn less_than_70pc() {
        run_test(50, 0);
    }
    #[test]
    fn less_than_100pc() {
        run_test(80, 1);
    }
    #[test]
    fn more_than_100pc() {
        run_test(120, 1);
    }

    #[test]
    #[should_panic]
    fn decreasing_value_panics() {
        let mock_messenger = MockMessenger::new();
        let mut lt = LimitTracker::new(&mock_messenger, 100);
        lt.set_value(50);
        lt.set_value(40);
    }
}
