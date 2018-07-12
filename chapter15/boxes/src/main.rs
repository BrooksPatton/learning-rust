use std::ops::Deref;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

use List::{Cons, Nil};

fn main() {
    let b = Box::new(5);

    println!("The box is {}", b);

    let list = Cons(1, 
        Box::new(Cons(2,
            Box::new(Cons(3, 
                Box::new(Nil))))));

    println!("The list is {:?}", list);

    let my_box = MyBox::new(55);
    let my_string_box = MyBox::new("hello world");

    println!("my box is {}", *my_box);
    println!("my string box is: {}", *my_string_box);
}
