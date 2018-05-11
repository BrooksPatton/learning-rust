fn main() {
    pushing_strings();
    concatenating_strings();
    using_format();
    iterating();
}

fn pushing_strings() {
    let mut hello = String::from("hello");
    let world = "world";
    let space = String::from(" ");
    let end = '!';
    // We are pushing the reference to the space String onto the end of hello
    hello.push_str(&space); 
    // Now we are tryng pushing a str to the end of hello
    hello.push_str(world);
    // All our variables are still usable.
    // push is used for characters
    hello.push(end);
    println!("the variable hello now is {}", hello);
    println!("the variable world is {}", world);
    println!("The space is still here as well |{}|", space);
}

fn concatenating_strings() {
    let hello = String::from("hello");
    let world = String::from(" world");
    let mut hello_world = hello + &world;
    let end = String::from("!");
    // This causes an error because we moved hello to hello_world
    // println!("hello {}", hello);
    // Another way is to use +=
    hello_world += &end;
    println!("hello world: {}", hello_world);
}

fn using_format() {
    let hello = String::from("hello");
    let world = String::from("world");
    let hello_world = format!("{} {}!", hello, world);
    println!("Using format: {}", hello_world);
    println!("We have not lost hello ({}) or world ({})", hello, world);
}

fn iterating() {
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    for b in "नमस्ते".chars() {
        println!("{}", b);
    }
}