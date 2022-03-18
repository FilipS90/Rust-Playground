#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width &&
        self.height > other.height
    }

    // associated func
    fn square(size: u32) -> Rectangle {
        Rectangle { height: size, width: size }
    }
}

fn main() {
    let example_rect = Rectangle {
        height: 30,
        width: 50,
    };

    println!("The are of the rectangle is {} square pixels.",
example_rect.area());

    let new_rect = Rectangle {
        height: 25,
        width: 48,
    };

    println!("Statement that new_rect can be fitted into example_rect is {}", example_rect.can_hold(&new_rect));

    let square = Rectangle::square(5);

    println!("{:?}", square);
    
}
