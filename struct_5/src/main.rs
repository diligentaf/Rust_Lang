struct User {
    active: bool,
    username: String, // &str won't work. Unless you use "Rust's lifetime"
    email: String,
    sign_in_count: u64,
} // stuct uses key: value pairs

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // impl block are called "Associated Functions"
    fn area(&self) -> u32 { // &self is basically an abbreviation of (rectangle : &Rectangle)
        self.width * self.height
    }

    fn width(&self) -> bool { // &self is basically an abbreviation of (rectangle : &Rectangle)
        self.width > 0
    }

    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

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

    //  == Calculating Rectangle ==
    println!();
    println!("== Calculating Rectangle ==");
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    //  == Refactoring with Tuples ==
    println!();
    println!("== Refactoring with Tuples ==");

    let rect1 = (30, 50);
    println!(
        "The area of the ractangle is {} square pixels.",
        area_tuple(rect1)
    );

    //  == Refactoring with Structs (win for clarity) ==
    println!();
    println!("== Refactoring with Structs (win for clarity) ==");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );

    //  == Priniting Struct using -> #[derive(Debug)] ==
    println!();
    println!("== Priniting Struct using -> #[derive(Debug)] ==");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1);
    dbg!(&rect1); // This debugging also works!. We don't want dbg! to take ownership so used reference

    //  == Defining Methods ==
    println!();
    println!("== Defining Methods ==");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!( "It is {}, that rectangle has a nonzero width", rect1.width());

    //  == Automatic Referencing and Dereferencing ==
    println!();
    println!("== Automatic Referencing and Dereferencing ==");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 80,
        height: 90,
    };
    let rect3 = Rectangle {
        width: 20,
        height: 40,
    };
    println!("{}",rect1.can_hold(&rect2));
    println!("{}",rect1.can_hold(&rect3));

    //  == Constructor in Struct ==
    println!();
    println!("== Constructor in Struct ==");
    dbg!(Rectangle::square(3)); // new struct instance has been created

}

fn build_user(email: String, username: String) -> User {
    User {
        email, // == email: email,
        username, // == username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}