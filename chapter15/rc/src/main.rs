use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    println!("Reference count for Rc(&a) - {}", Rc::strong_count(&a));
    let b = List::Cons(3, Rc::clone(&a));
    println!("Reference count for Rc(&a) - {}", Rc::strong_count(&a));
    {
        let c = List::Cons(4, Rc::clone(&a));
        println!("Reference count for Rc(&a) - {}", Rc::strong_count(&a));
    }
    println!("Reference count for Rc(&a) - {}", Rc::strong_count(&a));
}
