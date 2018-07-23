use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut counting_threads = Vec::new();

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let t = thread::spawn(move || {
            let mut c = counter.lock().unwrap();

            *c += 1;
        });

        counting_threads.push(t);
    }

    for t in counting_threads {
        t.join().unwrap();
    }

    println!("the total number is {}", *counter.lock().unwrap());
}
