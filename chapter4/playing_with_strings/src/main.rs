fn main() {
    let s1 = String::from("hello world!");
    print_string(&s1);
    
    println!("{}", s1);
}

fn print_string(string: &str) {
    let another_string = *string;
    println!("{}", another_string);
}