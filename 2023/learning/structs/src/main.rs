fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }


    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("{:#?}", user1.username);

    let user2 = User {
        username: String::from("teste"),
        ..user1
    };

    println!("{:#?}", user1.username);
    println!("{:#?}", user1.email);
    println!("{:#?}", user2.username);
}
