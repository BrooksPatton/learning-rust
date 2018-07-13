use std::ops::Deref;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T: std::fmt::Pointer>(T);

impl<T> MyBox<T> where T: std::fmt::Pointer {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> where T: std::fmt::Pointer {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Drop for MyBox<T> where T: std::fmt::Pointer {
    fn drop(&mut self) {
        println!("Dropping the smart pointer MyBox: {:p}", self.0);
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

    // let my_box = MyBox::new(55);
    let my_string_box = MyBox::new("hello world");
    // let my_other_string_box = MyBox::new(String::from("hello"));

    // println!("my box is {}", *my_box);
    println!("my string box is: {}", *my_string_box);
    println!("Memory location of my_box is {:p}", *my_string_box);

    // print_the_box(my_other_string_box);
    print_the_box(my_string_box);
}

fn print_the_box(b: MyBox<&str>) {
    println!("My box is: {}", *b);
}