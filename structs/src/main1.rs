struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {

    struct Color(i32, i32, i32);
    struct Print(i32, i32, i32);

    let mut user1 = User{
        email: String::from("dhirajbari911@gmail.com"),
        username: String::from("dhiraj0911"),
        active: true,
        sign_in_count: 1
    };

    let name = user1.username;

    user1.username = String::from("new username");
    
    let user2 = build_user(
        String::from("new username passed"),
        String::from("new email passed")
    );

    let user3 = User{
        email: String::from("email"),
        username: String::from("dhiraj"),
        ..user2 //from this it will get same value from user2 for remaining


    };
}


fn build_user(email: String, username: String) -> User{
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}