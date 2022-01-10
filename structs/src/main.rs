struct User {
    username: String,
    email: String,
    sign_ins: u32,
    active: bool,
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let user = User {
        username: String::from("Cofi"),
        email: String::from("filips@yahoo.com"),
        active: true,
        sign_ins: 1
    };


    let name: &str = &user.username[..];

    let user1 = build_user(
        String::from("fs@yahoo.com"), 
        String::from("Fic")
    );

    println!("{} and {}",name, user.username);

    let rent0 = Rectangle {
        height: 10,
        width: 8
    };

    let rectangle_size : u32 = rent0.area();

    let rect1: Rectangle = Rectangle {
        height: 15,
        width: 8
    };

    let bool = rect1.can_hold(rent0);
    print!("The rect1 rectangle can hold rect0 {}", bool);

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_ins: 1,
    }
}
