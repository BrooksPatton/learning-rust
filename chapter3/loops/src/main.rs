fn main() {
    let numbers = [0, 1, 2, 3];

    let a = for num in numbers.iter() {
        println!("{}", num);
    }
}
