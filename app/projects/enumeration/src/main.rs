enum Message {
    Write(String)
}

#[derive(Debug)] // すぐに州を点検できるように
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


impl Message {
    fn call(&self) {
        println!("world!");
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    let _some_number = Some(5);
    let _some_string = Some("a string");
    
    let _absent_number: Option<i32> = None;

    let _penny = Coin::Penny;
    let _nickel = Coin::Nickel;
    let _dime = Coin::Dime;
    let quarter_alabama = Coin::Quarter(UsState::Alabama);
    let _quarter_alaska = Coin::Quarter(UsState::Alaska);
    value_in_cents(quarter_alabama);

    // Option<T>とのマッチ
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    // if letで簡潔な制御フロー
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}