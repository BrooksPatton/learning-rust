#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self, my_str: &str) -> u32 {
        println!("{}", my_str);
        self.height * self.width
    }
}

fn main() {
    // let height = 15;
    // let width = 23;
    // let dimensions = (30, 50);

    let my_rectangle = Rectangle{
        height: 30,
        width: 50,
    };

    // let area = calculate_area(&my_rectangle);

    println!("The area is {}", my_rectangle.area("hello world"));
    println!("{:#?}", my_rectangle);
}

// fn calculate_area(rectangle: &Rectangle) -> u32 {
//     rectangle.height * rectangle.width
// }
