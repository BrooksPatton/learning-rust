pub mod structure {
  pub struct Guess {
    value: u8,
    pub other: u8,
  }

  impl Guess {
      pub fn new(number: u8) -> Guess {
          if number < 1 || number > 100 {
              panic!("You entered {}, but you can only use numbers between 1 and 100", number);
          }

          Guess {
              value: number,
              other: number,
          }
      }

      pub fn value(&self) -> u8 {
          self.value
      }
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}