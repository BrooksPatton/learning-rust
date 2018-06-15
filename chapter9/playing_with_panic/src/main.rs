extern crate playing_with_panic;
use playing_with_panic::structure;

fn main() {
  let mut guess = structure::Guess::new(55);

  guess.other = 72;

  println!("{}, {}", guess.value(), guess.other);
}