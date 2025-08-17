// match expression is used to compare a value against a series of patterns and execute code based on which pattern matches.
// very similar to if else chain

#![allow(dead_code)]

enum Coin {
    Gold,
    Silver,
    Bronze,
    Nickel,
    Quarter(UsState), // using the use state enum here
}

#[derive(Debug)]
enum UsState {
    // used above in coin enum
    Alabama,
    Alaska,
    California,
    Colorado,
}

fn main() {
    let coin = Coin::Gold;
    let value = value_in_cents(coin);
    println!("The value in cents is: {}", value);

    value_in_cents(Coin::Quarter(UsState::California));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Gold => {
            println!("Luckiest gold coin");
            100
        }
        Coin::Silver => 5,
        Coin::Bronze => 10,
        Coin::Nickel => 50,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

// comnbining match expression with enum -> go to file math+enum.rs
