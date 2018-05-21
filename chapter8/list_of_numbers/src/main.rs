// Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
extern crate list_of_numbers;
use list_of_numbers::data_structures::init;

fn main() {
    let mut list = init(vec![5, 4, 3, 2, 1, 99, 33, 67, 67, 12]);

    println!("The mean of the list is {}", list.mean());
    println!("The median of the list is {}", list.median());
    println!("The mode of the list is {}", list.mode());
}