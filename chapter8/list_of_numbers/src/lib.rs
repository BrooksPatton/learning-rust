
pub mod data_structures {
    use std::collections::HashMap;

    pub struct Numbers(Vec<i32>);

    impl Numbers {
        pub fn mean(&self) -> f32 {
            let sum: i32 = self.0.iter().sum();
            sum as f32 / self.0.len() as f32
        }

        pub fn median(&mut self) -> f32 {
            self.0.sort();
            
            let middle_index = self.0.len() / 2;

            // Thanks for Aeveris for the suggestion of match and just doing the math here 
            //  https://gist.github.com/aeveris/521aa7e2d0f006835537d563a932ee7c
            match self.0.len() % 2 {
                0 => (self.0[middle_index - 1] + self.0[middle_index]) as f32 / 2.0,
                _ => self.0[middle_index] as f32
            }
        }

        pub fn mode(&self) -> i32 {
            let mut numbers = HashMap::new();

            for number in self.0.iter() {
                numbers.entry(number)
                    .and_modify(|count| {*count += 1})
                    .or_insert(1);
            }

            let mut result = 0;
            let mut count = 0;

            numbers.iter().for_each(|(number, total)| {
                if total > &count {
                    count = *total;
                    result = **number;
                }
            });

            result
        }
    }

    pub fn init(list: Vec<i32>) -> Numbers {
        Numbers(list)
    }
}
