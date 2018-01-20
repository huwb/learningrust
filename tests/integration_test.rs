extern crate learningrust;

#[test]
fn longest_string_works() {
    assert_eq!(
        learningrust::longest_string::longest("yo", "yoyoyo"),
        "yoyoyo"
    );
}

#[test]
fn check_thread_pool() {
    use learningrust::threadpool::ThreadPool;
    use std::time::Duration;
    use std::thread::sleep;
    use std::sync::Arc;
    use std::sync::Mutex;

    #[derive(Debug)]
    struct EventTracker {
        thread_hits: Vec<usize>,
    }

    impl EventTracker {
        fn record_hit(&mut self, thread_id: usize) {
            self.thread_hits.push(thread_id);
        }
    }

    let et = Arc::new(Mutex::new(EventTracker {
        thread_hits: vec![],
    }));

    let tp = ThreadPool::new(2);

    // kick off some jobs

    // will get a thread and start immediately
    let et0 = Arc::clone(&et);
    tp.dispatch(Box::new(move || {
        sleep(Duration::from_millis(900));
        let mut et = et0.lock().unwrap();
        et.record_hit(0);
    }));
    // will get a thread and start immediately, will finish BEFORE first job
    let et1 = Arc::clone(&et);
    tp.dispatch(Box::new(move || {
        sleep(Duration::from_millis(500));
        let mut et = et1.lock().unwrap();
        et.record_hit(1);
    }));
    // has a short timer, but will not get a thread immediately, will finish second
    let et2 = Arc::clone(&et);
    tp.dispatch(Box::new(move || {
        sleep(Duration::from_millis(100));
        let mut et = et2.lock().unwrap();
        et.record_hit(2);
    }));

    // wait for jobs to finish processing
    tp.join();

    let et = et.lock().unwrap();

    assert_eq!(et.thread_hits.len(), 3);
    assert_eq!(et.thread_hits[0], 1);
    assert_eq!(et.thread_hits[1], 2);
    assert_eq!(et.thread_hits[2], 0);
}
