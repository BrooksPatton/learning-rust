fn main() {
    struct Doggies {
        name: String,
        age: u32
    };

    let kiki = Doggies {
        name: String::from("Kiki"),
        age: 5
    };

    println!("{} is {} years old", kiki.name, kiki.age);
}
