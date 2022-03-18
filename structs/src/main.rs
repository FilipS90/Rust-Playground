#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_ins: u32,
    active: bool,
}

struct AlwaysEqual;

fn main() {
    
    let user = User {
        username: String::from("Filip"),
        email: String::from("fic@google.com"),
        sign_ins: 1,
        active: true,
    };

    let new_user = User { username: String::from("Milos"), email: String::from("losmi@ggml"), ..user };

    println!("{:?}", new_user);
    println!("{:?}", user);

}

fn build_user(username: String, email: String) -> User {
    User { username, email, sign_ins: 1, active: true }
}
