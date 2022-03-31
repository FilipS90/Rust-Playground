fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    println!("{:?}", four);

    let six = IpAddrKind::V6(String::from("::1"));
    println!("{:?}", six);

    let m = Message::Write(String::from("Hello there"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("hi");
    let none_string: Option<u32> = None;

    let x = 5;
    let y = Some(3);
    let sum: u32 = {
        x + match y {
            None => 0,
            Some(i) => i,
        }
    };
    println!("The sum is {}", sum);

    let quarter = Coin::Quarter(UsState::Nevada);

    println!("{}", value_in_cents(quarter));

    let dice_roll: u8 = 2;
    let roll = match dice_roll {
        3 => println!("You got 3 !"),
        7 => println!("You got 7 !"),
        _ => (),
    };

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Nevada
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny !");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("This coin is from {:?} series", state);    
            25
        }
    }
}
