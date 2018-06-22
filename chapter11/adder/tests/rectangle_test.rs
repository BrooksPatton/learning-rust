extern crate adder;
use adder::shapes;

pub fn setup() {
  println!("{}", "Hi there");
}

#[test]
#[should_panic(expected="The rectangle you are creating is too small")]
fn rectangle_cannot_be_too_small() {
  shapes::Rectangle::new(1, 2);
}