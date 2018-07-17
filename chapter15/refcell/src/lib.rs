pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if self.is_above_75_percent(percentage_of_max) {
            self.messenger.send("Warning, you have used over 75% of your quota");
        } 
        else if self.is_above_90_percent(percentage_of_max) {
            self.messenger.send("Urgent warning, you have used over 90% of your quota");
        } 
        else if self.is_above_100_percent(percentage_of_max) {
            self.messenger.send("Error, you are over your quota");
        }
    }

    fn is_above_75_percent(&self, percent: f64) -> bool {
        percent >= 0.75 && percent < 0.9
    }

    fn is_above_90_percent(&self, percent: f64) -> bool {
        percent >= 0.9 && percent < 1.0
    }

    fn is_above_100_percent(&self, percent: f64) -> bool {
        percent >= 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(Vec::new()),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_over_75_percent_message() {
        let messenger = MockMessenger::new();
        let mut tracker = LimitTracker::new(&messenger, 100);
        tracker.set_value(80);

        assert_eq!(messenger.sent_messages.borrow().len(), 1);
    }
}
