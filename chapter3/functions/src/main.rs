fn main() {
    // println!("Hello, world!");

    let player_position = (35, 42);

    // another_function(player_position);

    // let x = {
    //     let y = 5;

    //     y + 1
    // };

    let x = another_function(player_position);

    println!("{}", x);
}

fn another_function(position: (u32, u32)) -> u32 {
    let (x, y) = position;
    println!("in the function: {}, {}", x, y);
    x
}