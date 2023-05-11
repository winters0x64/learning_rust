struct User {
    username:String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("ok@gmail.com"),
        username: String::from("ok"),
        sign_in_count: 2,
        active: true
    };

    println!{"{}",user1.email};
}