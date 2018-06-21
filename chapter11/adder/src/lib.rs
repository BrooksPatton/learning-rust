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

#[cfg(test)]
mod tests {
    #[test]
    fn canary_test() {
        assert_eq!(2 + 2, 4);
    }
}
