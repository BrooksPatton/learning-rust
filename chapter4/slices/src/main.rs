fn main() {
    //converting space to bytes
    // println!("{}", b' ');

    // let hello = "hello world!";
    // let h = &hello[..];
    // println!("{}", h);

    let a = [0, 1, 2, 3];
    // :? is "debug print"
    println!("{:?}", a);
    // :#? is "debug pretty print"
    println!("{:#?}", a);
    // for more formatting rules, check out https://doc.rust-lang.org/std/fmt/
    
    
    // the thing you are trying to print has to be debug-printable:
    struct NotPrintable { internal: u32 };
    let not_printable = NotPrintable { internal: 5 };
    // println!("{:?}", not_printable); // won't work
    
    
    #[derive(Debug)] // macro to generate code to make "Printable" implement the "Debug" trait
    struct Printable { internal: u32 };
    
    let printable = Printable { internal: 5 };
    println!("{:?}", printable); // shows "Printable { internal: 5 }"
    println!("{:#?}", printable); // shows: "Printable {
                                  //              internal: 5
                                  //         }"
}
