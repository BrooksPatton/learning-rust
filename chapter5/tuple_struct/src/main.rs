struct Counts(u32, u32, u32);

fn main() {

    let random_numbers = Counts(13, 99, 23);
    let all_are_one = convert_to_one(random_numbers);

    println!("{:?}", all_are_one);
}

fn convert_to_one(t: Counts) -> (u32, u32, u32) {
    (1, 1, 1)
}