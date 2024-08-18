// enum IpAddressKind {
//     v4,
//     v6,
// }
/*
 fn main() {
    let a = Some(1);
    let b = Some("str");
    let c: Option<i32> = None;
}
\*/
#[derive(Debug)]
enum UsState {
    Alaska,
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    // let coin = Coin::Penny;
    let coin = Coin::Quarter(UsState::Alaska);
    match coin {
        Coin::Penny => {
            println!("{:#?}", Coin::Penny);
            1
        }
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter(state) => {
            println!("{:#?}", state);
            4
        }
    };
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
fn plus_one(option: Option<i32>) -> Option<i32> {
    match option {
        // None => None,
        Some(i) => Some(i + 1),
        _ => None,
    }
}
