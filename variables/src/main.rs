use std::io;

fn main() {
    // Rust’s naming convention for constants is to use all uppercase with underscores between words.
    const X: i32 = 4;
    println!("The value of x is: {X}");

    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    let x = 5;
    // Shadowing : shadowing changes the value when let is declared
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {x}");
    }

    println!("The value of x is : {x}");

    // *Note* : this won't work with mut because mut requires same type. But in this shadowing case, the type also changes by declare let
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");

    // Rust is a statically typed language, which means that it must know the types of all variables at compile time.
    let _guess: u32 = "42".parse().expect("Not a number!");

    // Types
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0

    // remainder
    let _remainder = 43 % 5;

    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    // == Tuple ==
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!(
        "five_hundred = {}, six_point_four = {}, one = {} ",
        x.0, x.1, x.2
    );

    // == Array ==
    // Arrays are useful when you want your data allocated on the stack rather than the heap
    // An array isn’t as flexible as the vector type, though
    let a = [1, 2, 3, 4, 5];
    println!("{}", a[1]);

    // initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon,
    // and then the length of the array in square brackets, as shown here:
    let a = [3; 5];
    // 👆 let a = [3, 3, 3, 3, 3]; but in a more concise way

    // Arrays are more useful when you know the number of elements will not need to change
    // For example, if you were using the names of the month in a program,
    // you would probably use an array rather than a vector because you know it will always contain 12 elements

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a = [1, 2, 3, 4, 5];

    println!("{}", a[4]);

    // == Invalid Array Element Access ==
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index (type in 3) : ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    // == Callling a function ==
    another_function(99, 'C');

    // let x = (let y = 6);

    let y = {
        let x = 3;
        x + 9876
    };
    println!("{y}");
    // Expressions do not include ending semicolons.
    // If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value

    let x = return_five();
    println!("The value of x (returned from the function) is : {x}");

    let x = plus_one(36);
    println!("The value of x (returned from plus_one function) is : {x}");

    // == if statements ==

    // The condition in this code must be a bool
    let number = 7;
    if number < 5 {
        println!("the value is less than 5 and the number is : {number}");
    } else {
        println!("the valide is more than or equal to 5 and the number is : {number}");
    }

    // multiple conditions
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if and a let Statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // // If the types are mismatched, as in the following example, we’ll get an error:
    // // This won’t work because variables must have a single type,
    // // and Rust needs to know at compile time what type the number variable is, definitively
    // let condition = false;
    // let number = if condition { 5 } else { "six" };
    // println!("The value of number is: {number}");

    // == Loop ==
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn another_function(x: i32, unit_label: char) {
    println!("calling another function");
    println!("The value of x is : {x} - {unit_label}")
}

fn return_five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
