use std::thread;
use std::sync::mpsc;
use std::time;

pub fn run() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        println!("[SPAWNED] Typing message from spawned thread...");
        thread::sleep(time::Duration::from_millis(5000));
        let val = String::from("Hellooooo from the other thread..");
        println!("[SPAWNED] Sending message!");
        tx.send(val).unwrap();
    });

    let mut got_it = false;

    while !got_it {
        match rx.try_recv() {
            Ok(result) => {
                println!("[MAIN] Message received!");
                println!("[MAIN] Message: {:?}", result);
                got_it = true;
            }
            Err(details) => {
                println!("[MAIN] Got an error: {}", details);
                thread::sleep(time::Duration::from_millis(1000));
            }
        };
    }
}
