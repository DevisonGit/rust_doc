fn main() {
    // criando uma instancia de uma struct
    let user1 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("username: {0}", user1.username);

    // criando uma instancia mutavel
    let mut user2 = User {
        active: true,
        username: String::from("user-2"),
        email: String::from("user2@examples.com"),
        sign_in_count: 1,
    };


    println!("email: {0}", user2.email);
    println!("change email");
    user2.email = String::from("user2other@example.com");
    println!("email: {0}", user2.email);
    
    // criando uma funçao que retorna uma struct
    println!();
    let mut user3 = build_user(String::from("user3"), String::from("user3@example.com"));
    println!("{}", user3.email)

}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}