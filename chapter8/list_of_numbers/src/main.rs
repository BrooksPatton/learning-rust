// Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.

fn main() {
    let list = vec![1, 2, 3, 4, 5];

    println!("The mean of the list is {}", get_mean(&list));
}

fn get_mean(list: &Vec<i32>) -> f32 {
    let mut accumulator = 0.0;

    for number in list.iter() {
        accumulator += *number as f32;
    }

    accumulator / list.len() as f32
}