use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let messages = vec![
            String::from("first"),
            String::from("second"),
            String::from("third"),
            String::from("fourth"),
        ];

        for message in messages {
            tx.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let messages = vec![
            String::from("another"),
            String::from("set"),
            String::from("of"),
            String::from("messages"),
        ];

        for message in messages {
            tx1.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("{}", received);
    }
}
