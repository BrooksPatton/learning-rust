use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];
    let slow_count = move || {
        for i in 1..10 {
            println!("Hello from the thread. {}", i);
            thread::sleep(Duration::from_millis(1));
        }

        println!("The vector is: {:?}", v);
    };

    let my_thread = thread::spawn(slow_count);

    for i in 1..5 {
        println!("I am not from a thread. {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    my_thread.join().unwrap();
}