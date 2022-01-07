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
    loop_through_array();
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

    while var != 0 {
        println!("{}", var);
        var -= 1;
    }

    println!("We have liftoff !");

    return var;
}

fn loop_through_array() {
    let arr = [1,2,3,4,5];
    // let mut index = 0;

    // while index < 5 {
    //     println!("For index of {}, the value in array is {}", index, arr[index]);
    //     index += 1;
    // }

    for element in arr {
        println!("Current element in iteration {}", element);
    }
}
