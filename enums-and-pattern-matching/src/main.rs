use std::option;

fn main() {
    let four = IpAddressKind::V4(100,5,10,95);

    let nothing = Message::Quit;

    let some_number = Some(1235);
    let real_number = some_number.unwrap_or(0);
    let num = 23;
    let sum = real_number + num;

    println!("{:?}", sum);

    println!("{:?}", four);
    println!("{:?}", nothing);

    let message = Message::Write(String::from("Blah"));
    message.call();

    let coin_val = value_in_cents(Coin::Penny);

    println!("{:?}", coin_val);

    let dice_roll = 5;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => take_off_hat(),
        _ => move_and_reroll(),
    }

}

#[derive(Debug)]
enum IpAddressKind {
    V4(u8, u8, u8, u8)
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn add_fancy_hat() {
    println!("Added fancy hat!");
}

fn take_off_hat(){
    println!("Took off fancy hat");
}

fn move_and_reroll(){
    println!("Move and then reroll");
}
