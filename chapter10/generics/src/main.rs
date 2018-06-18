use std::fmt::Debug;

trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Points<T, U, A> {
    x: A,
    y: U,
    z: T,
}

impl<T, U, A> Points<T, U, A> {
    fn x(&self) -> &A {
        &self.x
    }
}

impl Points<i32, i32, i32> {
    fn squared_x(&self) -> i32 {
        self.x * self.x
    } 
}

impl<T, U, A> Summary for Points<T, U, A>      
    where T: Debug,
          U: Debug,
          A: Debug
{
    fn summarize(&self) -> String {
        format!("x is {:?}, y is {:?}, and z is {:?}", self.x, self.y, self.z)
    }
}

fn find_largest<T: PartialOrd+Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for i in 1..list.len() {
        let current = list[i];
        if current > largest {
            largest = current;
        }
    }

    largest
}

fn main() {
    let numbers = vec![3, 5, 9, 15];
    let largest_number = find_largest(&numbers);
    println!("The largest number is {}", largest_number);

    let characters = vec!['a', 'p', 'z'];
    let largest_char = find_largest(&characters);
    println!("The largest character is {}", largest_char);

    let coordinates = Points {x: 15, y: 64.0, z: "Hello world".to_string()};
    let coordinates_i32 = Points {x: 2, y: 4, z: 6};

    println!("coordinate: {:?}", coordinates);
    println!("the x is {}", coordinates.x());
    println!("x squared is {}", coordinates_i32.squared_x());
    println!("summized: {}", coordinates.summarize());
}
