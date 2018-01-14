use std::thread;
use std::sync::mpsc;
use std::time;

pub fn run() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        println!("[SPAWNED] Typing message from spawned thread...");
        thread::sleep(time::Duration::from_millis(3000));
        let val = String::from("Hellooooo from the other thread..");
        println!("[SPAWNED] Sending message!");
        tx.send(val).unwrap();
        println!("[SPAWNED] Goodbye!");
    });

    for received in rx {
        println!("[MAIN] Message received: {:?}", received);
    }

    println!("[MAIN] Channel terminated");
}
