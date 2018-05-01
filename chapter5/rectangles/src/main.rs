#![allow(dead_code)]

#[derive(Debug)]
struct Rectangle {
    name: String,
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_fit_inside(&self, rect: &Rectangle) -> bool {
        rect.height < self.height && rect.width < self.width
    }

    fn create_square(size: u32, name: &str) -> Rectangle {
        Rectangle{name: String::from(name), width: size, height: size}
    }

    fn unused(&self) -> bool {
        true
    }
}

fn main() {
    // let height = 15;
    // let width = 23;
    // let dimensions = (30, 50);

    let rect1 = Rectangle{height: 30, width: 50, name: String::from("rect1")};
    let rect2 = Rectangle{height: 10, width: 40, name: String::from("rect2")};
    let rect3 = Rectangle{height: 60, width: 45, name: String::from("rect3")};

    println!("The area of {} is {}", rect1.name, rect1.area());
    // println!("{:#?}", my_rectangle);

    println!("{} can fit inside {}: {}", rect2.name, rect1.name, rect1.can_fit_inside(&rect2));
    println!("{} can fit inside {}: {}", rect3.name, rect1.name, rect1.can_fit_inside(&rect3));

    let my_square = Rectangle::create_square(32, "my square");

    println!("{:?}", my_square);
}

// fn calculate_area(rectangle: &Rectangle) -> u32 {
//     rectangle.height * rectangle.width
// }
