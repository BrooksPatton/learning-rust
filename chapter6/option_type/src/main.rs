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
    let total_value = Some(0);

    println!("My coin is worth {} cents", get_coin_value(&my_penny));
    println!("My coin is worth {} cents", get_coin_value(&shiny_nickel));
    println!("My coin is worth {} cents", get_coin_value(&quarter));

    let total_value = add_coin_value(get_coin_value(&my_penny), total_value);
    let total_value = add_coin_value(get_coin_value(&shiny_nickel), total_value);
    let total_value = add_coin_value(get_coin_value(&quarter), total_value);

    println!("All my coins add up to {} cents", total_value.unwrap());
}

fn get_coin_value(coin: &Coin) -> u32 {
    match *coin { // destructuring coin
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(ref state) => { // borrow within context of destructuring
            println!("You got state {:?}", state);
            25
        },
    }
}

fn add_coin_value(cents: u32, total_value: Option<i32>) -> Option<i32> {
    match total_value {
        None => None,
        Some(current_cents) => Some(cents as i32 + current_cents),
    }
}