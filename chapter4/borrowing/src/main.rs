fn main() {
    let mut s = String::from("hello world");

    let s2 = &mut s;

    // Invalid as s is borrowed mutably twice
    {
        let s1 = &mut s;
    }


    println!("{}", s2);
}
