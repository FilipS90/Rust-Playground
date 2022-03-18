fn main() {
    let x = (1,5,"Hello", 'a');
    println!("{}", x.3);

    let q = [3; 5];
    println!("{}", q[2]);

    let number : u32 = 30;

    if number > 40 {
        println!("condition was true");
    } else if number < 100 {
        println!("blah");
    } else {
        println!("condition was false");
    }

    println!("The result of someMethod is: {}", some_method());
}
