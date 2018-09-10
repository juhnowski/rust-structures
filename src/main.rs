fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: i64,
        active: bool,
    }

    let user1 = User {
        username:   String::from("Juhnowski"),
        email:      String::from("juhnowski@gmail.com"),
        sign_in_count: 0,
        active: true,
    };

    println!("username={} email={} sign_in={} active={}", user1.username, user1.email, user1.sign_in_count,user1.active);
}
