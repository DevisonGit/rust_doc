#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("State quater from {state:?}!");
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quater(UsState::Alabama));
    value_in_cents(Coin::Quater(UsState::Alaska));
    value_in_cents(Coin::Penny);

    // match padrão
    let five = Some(5);
    let six = plus_one(five);
    println!("the value of six is: {six:?}");
    let none = plus_one(None);
    println!("the value of variable none: {none:?}")
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

