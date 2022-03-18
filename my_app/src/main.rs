use rand::Rng;

fn main() {
    let num = some_method();
    println!("Hello some_method, you returned: {}", num);
}

fn some_method() -> i32 {

    let mut count = 0;
    let mut attempts = 0;
    loop {
        let random_number = rand::thread_rng().gen_range(1..6);

        if random_number % 5 == 0 {
            println!("random number is {}", random_number);
            count += 1;
        }

        if count == 3 {
            return count;
        }

        if attempts == 12 {
            return 0;
        }

        attempts += 1;
    }
}
