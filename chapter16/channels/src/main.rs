use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
        let value = String::from("beginning");

    thread::spawn(move || {
        tx.send(&value).unwrap();
        for _i in 1..6 {
            tx.send(&String::from("...waiting")).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        println!("the first value was {}", &value);
    });

    loop {
        match rx.recv() {
            Ok(message) => println!("got the message: {}", message),
            Err(e) => {
                println!("error: {:?}", e);
                break;
            },
        };
    }
}
