enum UsState {
    Alabame,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_enums(coin : Coin) -> u32 {
    match coin {
        coin::Penny => 1,
        coin::Nickel => 5,
        coin::Dime => 10,
        coin::Quarter(state) => {
            println!("{}", state);
            25
        },
    }
}

fn sum_value(x: Option<u32>) -> Option<u32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[test]
fn test_sum_match_value () {
    println!("{}", sum_value(Some(1)));
}
