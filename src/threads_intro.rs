use std::thread;
use std::sync::mpsc;
use std::time;

pub fn run() {
    let (tx, rx) = mpsc::channel();

    for i in 1..10 {
        let txi = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            println!("[SPAWNED{}] Typing message from spawned thread...", i);
            thread::sleep(time::Duration::from_millis(3000));
            let val = format!("Hellooooo from thread {}", i);
            println!("[SPAWNED{}] Sending message!", i);
            txi.send(val).unwrap();
            println!("[SPAWNED{}] Goodbye!", i);
        });
    }
    // needed to close all channels. not very nice but I wasn't able to nicely clone
    // for 9 threads after trying a couple of approaches.
    drop(tx);

    for received in rx {
        println!("[MAIN] Message received: {:?}", received);
    }

    println!("[MAIN] Channel terminated");
}
