// Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
use std::collections::HashMap;

#[derive(Debug)]
enum ListOfNumbers {
    Numbers(Vec<i32>)
}

impl ListOfNumbers {
    fn mean(&self) -> f32 {
        println!("{:?}", self);
        33.0
    }
}

fn main() {
    // let list = vec![5, 4, 3, 2, 1];
    let list = ListOfNumbers::Numbers(vec![5, 4, 3, 2, 1]);

    // println!("The mean of the list is {}", get_mean(&list));
    println!("The mean of the list is {}", list.mean());
    // println!("The median of the list is {}", get_median(&list));
    // println!("The mode of the list is {}", get_mode(&list));
    // println!("The original vector is {:?}", list);
}

fn get_mean(list: &[i32]) -> f32 {
    let mut accumulator = 0.0;

    for number in list.iter() {
        accumulator += *number as f32;
    }

    accumulator / list.len() as f32
}

fn get_median(list: &Vec<i32>) -> f32 {
    let mut sorted_list = list.clone();
    let length = sorted_list.len();
    let is_odd = length % 2 != 0;
    let middle_index = length / 2;

    sorted_list.sort_unstable();

    if is_odd {
        return sorted_list[middle_index] as f32;
    } else {
        let middle_two = &sorted_list[middle_index - 1..middle_index + 1];
        let mean = get_mean(middle_two);
        return mean;
    }
}

fn get_mode(list: &[i32]) -> i32 {
    let mut numbers = HashMap::new();

    for number in list.iter() {
        let count = numbers.entry(number).or_insert(0);
        *count += 1;
    }

    let mut mode = (0, 0);
    for (number, count) in numbers.iter() {
        if count > &mode.1 {
            mode.0 = **number;
            mode.1 = *count;
        }
    }

    mode.0
}