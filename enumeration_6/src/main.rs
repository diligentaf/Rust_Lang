#![allow(unused)]

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrConstructor {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrTuple {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum IPAddrKind {
    V4,
    V6(u8, u8, u8),
    V7(String),
    V8 { x: u32, y: u32 },
}

#[derive(Debug)]
struct IPAddr {
    kind: IPAddrKind,
    address: String,
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

    //  == From Youtube ==
    println!();
    println!("== From Youtube ==");
    // https://www.youtube.com/watch?v=JKmkKae-EhM
    let five = Some(5);
    let six = plus_one(five);
    // let none: Option<Option<i32>> = Some(None); // don't know why it doesn't work
    let none = Some(Option::<i32>::None);

    let index = 6;
    let name = String::from("Domenic");
    println!(
        "Character at index {}: {}",
        &index,
        match name.chars().nth(index) {
            Some(c) => c.to_string(),
            None => "No character at index 8!".to_string(),
        }
    );

    println!(
        "Occupation is {}",
        match get_occupation("Freddy") {
            Some(o) => o,
            None => "No occupation found!",
        }
    );

    let address1 = IPAddr {
        kind: IPAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let address2 = IPAddr {
        kind: IPAddrKind::V6(11, 22, 33),
        address: String::from("::1"),
    };

    let address3 = IPAddr {
        kind: IPAddrKind::V7(String::from("Currently listening to Ghost")),
        address: String::from("::1"),
    };

    let address4 = IPAddr {
        kind: IPAddrKind::V8 { x: 30, y: 40 },
        address: String::from("::1"),
    };

    println!("{:?}", address1.kind);
    println!("{:?}", address2.kind);
    println!("{:?}", address3.kind);
    println!("{:?}", address4.kind);

    dbg!(&address1.kind);
    dbg!(&address2.kind);
    dbg!(&address3.kind);
    dbg!(&address4.kind);

    //  == Pattern Matching (from Youtube) ==
    println!();
    println!("== Pattern Matching (from Youtube) ==");

    match &address2.kind {
        // match needs to cover all cases. will get an error if not covered
        IPAddrKind::V4 => println!("nothing happens"),
        IPAddrKind::V6(11, _, 33) => println!("it matches! passes here"),
        IPAddrKind::V6(a, b, c) => println!("{}.{}.{}", a, b, c),
        IPAddrKind::V7(s) => println!("{}", s),
        IPAddrKind::V8 { x, y } => {
            let z = x + y;
            println!("{}", z);
        }
        _ => println!("To cover all the uncovered cases, use this"),
    }

    // To cover a single case using let:
    if let IPAddrKind::V8 { x: 30, y: 40 } = address4.kind {
        println!("it matches address 4");
    }

    // MOOG: Don't know why this doesn't work
    // let num: Option<u32> = Some(5);
    // match num {
    //     Some(n) => println!("{}", n),
    //     None => println!("No value!"),
    // }

    //  == 6.3 Concise Control Flow with if let ==
    println!();
    println!("== 6.3 Concise Control Flow with if let ==");

    let config_max = Some(99u8);
    match config_max {
        Some(max) => println!("From match statement: The maximum is configured to be {}", max),
        _ => (),
    }
    // the above works.. and the below statement also works
    // but using the statement below loses the exhaustive checking that match enforces. Thus, there's a tradeoff
    if let Some(max) = config_max {
        println!("From if statement: The maximum is configured to be {}", max);
    }

    // let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    // // The Above code and the the below codes are basically the same

    // let mut count = 0;
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}!", state);
    // } else {
    //     count += 1;
    // }

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Domenic" => Some("Software Developer"),
        "Michael" => Some("Dentist"),
        _ => None,
    }
}

