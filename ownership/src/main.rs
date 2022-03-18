fn main() {
    
    let str = String::from("blah");

    let x = str;

    print!("{}", x);

    nesto();

    let word = String::from("Beograde Dobro Jutro");
    let first_word = first_word(&word);
    println!("{}", first_word);

    let q = built_to_work(&String::from("a"));
}

fn nesto(){
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let _r3 = &mut s; // no problem
}

fn first_word (s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &element) in bytes.iter().enumerate() {
        if element == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}

fn built_to_work (s: &String) -> &str {
    let x = &s[..s.len()];
    x
}
