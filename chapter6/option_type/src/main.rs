enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Colorado
}

fn main() {
    let my_penny = Coin::Penny;
    let shiny_nickel = Coin::Nickel;
    let state = UsState::Colorado;
    let quarter = Coin::Quarter(state);

    println!("My coin is worth {} cents", get_coin_value(my_penny));
    println!("My coin is worth {} cents", get_coin_value(shiny_nickel));
    println!("My coin is worth {} cents", get_coin_value(quarter));
}

fn get_coin_value(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("You got state {:?}", state);
            25
        },
    }
}