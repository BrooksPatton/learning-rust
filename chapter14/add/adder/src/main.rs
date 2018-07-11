extern crate add_one;
extern crate add_two;

fn main() {
    let num = 10;
    println!("Original number is {}, after adding one is {}", num, add_one::add_one(num));

    println!("Original number is {}, after adding two is {}", num, add_two::add_two(num));
}
