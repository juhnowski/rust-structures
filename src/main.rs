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

    let user2 = build(String::from("user"), String::from("password"));

    print_user(&user1);
    print_user(&user2);

    fn build(username: String, email:String) -> User {
            User{
                username,
                email,
                sign_in_count: 1,
                active: true,
            }
    };

    fn print_user(user: &User){
        println!("username={} email={} sign_in={} active={}", user.username, user.email, user.sign_in_count,user.active);
    }
}
