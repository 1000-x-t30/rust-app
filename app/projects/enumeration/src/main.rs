enum Message {
    Write(String)
}

#[derive(Debug)] // すぐに州を点検できるように
enum UsState {
    Alabama,
    Alaska,
    // ... などなど
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


impl Message {
    fn call(&self) {
        println!("{}", "world!");
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

    println!("{}", value_in_cents(quarter_alabama));
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