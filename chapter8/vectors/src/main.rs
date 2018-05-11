fn main() {
    let v: Vec<i32> = Vec::new();

    // this doesn't work, it needs the type annotations
    // let other_v = Vec::new(); 
    
    let auto_v = vec![1.0, 2.0, -33.0];

    // this works because we are pushing, and therefore Rust can figure out what goes inside
    let mut my_ints = Vec::new();
    my_ints.push(1);
    my_ints.push(2);
    // println!("{:?}", my_ints);

    // vectors will take ownership of whatever is put inside them. Therefore if you want to keep ownership, borrow the value and then give it to the vector. However we can also take back ownership if we want to.
    let my_number = String::from("hello world");
    let another_string;
    {
        let mut my_strings = Vec::new();
        // Here you push only a reference to `my_number` onto the Vec
        my_strings.push(&my_number);
        // Because of the reference you can just index into the Vec and move the value out of
        // it, the reference gets copied (immutable references are Copy types).
        // Try and move the String itself into the Vec, you will get an error that you cannot
        // move it out of the Vec like this, since it would leave the first element in an invalid
        // state.
        // If you want to take elements that do not implement the Copy Trait out of the Vec,
        // there are methods you can use, e.g. `pop()`, `remove()` or `swap_remove()`.
        another_string = my_strings[0];
        // The length is unchanged, since the reference to `my_number` is still in the Vec.
        println!("{}", my_strings.len());
    }
    println!("{}", another_string);
    
    let my_vector = vec![1, 2, 3, 4, 5];
    // let does_not_exist1 = &my_vector[100];
    let does_not_exist2 = my_vector.get(100);
    println!("{:?}", does_not_exist2);

    // We can iterate over all values in a vector easily
    for value in &my_vector {
        println!("{}", value);
    }

    // We can also loop over a vector and change its values if it is mutable
    let mut nums = vec![0, 1, 2];
    for num in &mut nums {
        *num += 10;
    }
    println!("{:?}", nums);
}
