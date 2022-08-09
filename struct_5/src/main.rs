struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
} // stuct uses key: value pairs

fn main() {
    //  == Creating Struct ==
    println!();
    println!("== Creating Struct ==");

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername1"),
        active: true,
        sign_in_count: 1,
    };
    println!("previous email : {}", user1.email);
    println!("{}", user1.username);
    println!("{}", user1.active);
    println!("{}", user1.sign_in_count);

    user1.email = String::from("anothernewemail@example.com");
    println!("new email : {}", user1.email);

    //  == Creating Struct Using Fuction ==
    println!();
    println!("== Creating Struct Using Fuction ==");
    let user1 = build_user(
        String::from("someoneuser1@example.com"),
        String::from("someoneuser1biatch"),
    );
    println!("build_user function user 1 email : {}", user1.email);
    println!("build_user function user 1 email : {}", user1.username);

    //  == Creating New User Instance Using One of The Values from User1 ==
    println!();
    println!("== Creating New User Instance Using One of The Values from User1 ==");
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("user2@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("user2.email : {}", user2.email);
    println!("user2.username : {}", user2.username);
    println!("user2.active : {}", user2.active);
    println!("user2.sign_in_count : {}", user2.sign_in_count);

    //  == Abbreviated Syntax Struct Update ==
    println!();
    println!("== Abbreviated Syntax Struct Update ==");
    // Using struct update syntax, we can achieve the same effect with less code
    let user3 = User {
        email: String::from("user3@example.com"),
        ..user2
    };
    println!("user3.email : {}", user3.email);
    println!("user3.username : {}", user3.username);
    println!("user3.active : {}", user3.active);
    println!("user3.sign_in_count : {}", user3.sign_in_count);

    //  == Tuple Struct ==
    println!();
    println!("== Tuple Struct ==");
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}", black.0);
    println!("{}", origin.0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // == email: email,
        username, // == username: username,
        active: true,
        sign_in_count: 1,
    }
}