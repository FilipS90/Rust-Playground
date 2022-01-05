fn main() {
    println!("Hello, world!");

    another_function(15);

    call_crazy();

    let five: i32 = five();

    println!("The result is {}", five);

    println!("{} plus one is {}", five, plus_one(five))
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32){
    println!("Yet another hello, and x is {} !",x);
}

fn five() -> i32 {
    5
}

fn call_crazy(){
    let x = {
        let q = 5;
        q + 10
    };

    println!("The value of crazy is {}", x);
}
