fn main() {
    
    let mut v: Vec<String> = Vec::new();

    v.push(String::from("Cofi"));
    v.push(String::from("Draga"));
    v.push(String::from("Johnny"));

    let v1 = vec![1,2,3,4,5];

    let third = &v1[2];
    let second = v1.get(1);
    let mut no_element = v1.get(20);

    println!("{}", no_element.get_or_insert(&0));

    let mut v2 = vec![1,2,3,4,5];

    // doesnt work, cant have v2.push() as mutable borrow
    // after immutable ref because of println after
    // let first = &v2[0];

    v2.push(6);
    // does work
    let first = &v2[0];

    print!("The first element is: {}", first);
    println!();

    for i in &v2 {
        print!("{} ", i);
    }

    println!();

    for i in &mut v2 {
        *i += 50;
    }

    println!();

    for i in &v2 {
        print!("{} ", i);
    }

    println!();

    enum SpreadshetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let enum_vector = vec! [
        SpreadshetCell::Int(3),
        SpreadshetCell::Float(10.33),
        SpreadshetCell::Text(String::from("Woah !"))
    ];

    for i in enum_vector {
        match i {
            SpreadshetCell::Int(i) => println!("Just an int"),
            SpreadshetCell::Float(i) => println!("Just a float"),
            SpreadshetCell::Text(i) => println!("Just a text")
        }
    }









    
}
