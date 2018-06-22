pub mod shapes {
    #[derive(Debug, PartialEq)]
    pub struct Rectangle {
        pub length: u32,
        pub width: u32,
    }

    impl Rectangle {
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.length > other.length && self.width > other.width
        }

        pub fn new(length: u32, width: u32) -> Rectangle {
            if length < 5 && width < 5 {
                panic!("The rectangle you are creating is too small, make it at least 5x5");
            }

            Rectangle {length, width}
        }
    }

    #[test]
    fn rectangle_struct() {
        let rect_small = Rectangle {length: 15, width: 15};
        let rect_large = Rectangle {length: 30, width: 30};

        assert!(rect_large.can_hold(&rect_small));
        assert!(!rect_small.can_hold(&rect_large))
    }

    #[test]
    fn it_is_equal() {
        let first = Rectangle {length: 10, width: 10};
        let second = Rectangle {length: 10, width: 10};
        assert_eq!(first, second);
    }

    #[test]
    #[should_panic(expected="The rectangle you are creating is too small, make it at least 5x5")]
    fn it_is_not_small() {
        Rectangle::new(1, 2);
    }
}

pub mod adders {
    pub fn add_two(number: i32) -> i32 {
        number + 2
    }

    #[test]
    fn add_two_test() {
        assert_eq!(add_two(2), 4);
        assert_eq!(add_two(4), 6);
    }
}

pub mod greeters {
    pub fn greet(name: &str) -> String {
        format!("Good morning {}", name)
    }

    #[test]
    fn greet_should_use_name() {
        let result = greet("mrecoelho");
        assert!(result.contains("mrecoelho"), format!("The name was not correct :(. we got '{}' instead", result));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn canary_test() {
        assert_eq!(2 + 2, 4);
    }
}
