use std::sync::Arc;
use std::thread;
use std::sync::Mutex;

pub fn run() {
    let mut handles = vec![];
    let m = Arc::new(Mutex::new(6));

    for i in 0..10 {
        let counter = Arc::clone(&m);
        handles.push(thread::spawn(move || {
            let mut data = counter.lock().unwrap();
            *data += 1;
            println!("[{}] Added: {}", i, data);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {:?}", m);
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn double_mutex_unlock_panics() {
//         let m = Mutex::new(6);
//         let guard1 = m.lock().unwrap();
//         let guard2 = m.lock().unwrap();
//     }
// }
