#![allow(unused)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrConstructor {
    V4(String),
    V6(String),
}

enum IpAddrTuple {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddrConstructor::V4(String::from("127.0.0.1"));
    let loopback = IpAddrConstructor::V6(String::from("::1"));

    // struct Ipv4Addr {}
    // struct Ipv6Addr {}

    // enum IpAddr {
    //     V4(Ipv4Addr),
    //     V6(Ipv4Addr),
    // }

    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    //  == The "Option" Enum and its Advantages over Null Values ==
    println!();
    println!("== Enum and its Advantages over Null Values ==");
    // rust doesn't have a NULL feature
    enum Option<T> {
        // Option<T> types a different type
        None,
        Some(T),
    }
    let some_number = Some(5);
    let some_string = Some("a string");
    // let absent_number: Option<i32> = None;

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y; // This doens't work. cuz i8 and Option<i8>'s types are different

    //  == The match Control Flow Construct ==
    println!();
    println!("== The match Control Flow Construct ==");
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            // match is similar to if statement but can return any type unlike if that returns boolean
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum CoinByState {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents_by_usa(coin: CoinByState) -> u8 {
        match coin {
            CoinByState::Penny => 1,
            CoinByState::Nickel => 5,
            CoinByState::Dime => 10,
            CoinByState::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
    // when we call the above function, it would be something like -> value_in_cents_by_usa(CoinByState::Quarter(UsState::Alaska))

    let five = Some(5);
    let six = plus_one(five);
    // let none = Some(None);

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}