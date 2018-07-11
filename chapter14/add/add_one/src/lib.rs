extern crate rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_adds_one() {
        assert_eq!(add_one(5), 6);
    }
}
