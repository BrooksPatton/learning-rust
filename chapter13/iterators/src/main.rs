#[derive(Debug)]
struct Counter {
    count: u8
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}

impl Iterator for Counter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 6 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let counter = Counter::new();
    let counter2 = Counter::new();


    let zipped = counter.zip(counter2.map(|num| num + 1));

    for c in zipped {
        println!("c is {:?}", c);
    }
    
}
