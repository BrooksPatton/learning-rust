mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {
            ::outermost::middle_secret_function();
        }

        fn secret_function() {}
    }
}

fn try_me() {
    use outermost::{middle_function, inside::{inner_function}};

    middle_function();
    outermost::middle_secret_function();
    inner_function();
    outermost::inside::secret_function();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
