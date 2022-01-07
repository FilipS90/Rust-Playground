fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    loops_func();

    values_from_loops(5);

    withwhile();
}

fn loops_func(){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn values_from_loops(a: u32) -> u16 {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{}", counter);

    return result;
}

fn withwhile() -> u32 {
    let mut var = 5;
    const name: String = "Filip";

    while var != 0 {
        println!("{}", var);
        var -= 1;
    }

    println!("We have liftoff !");

    return var;
}
