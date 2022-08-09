fn main() {
    //  == Understanding Stack ==
    println!();
    println!("== Understanding Stack ==");
    // Both stack and heap are parts of memory available to your code to use at runtime.
    // Stack stores valuesin the order it gets them and removes then values in the opposite order.
    // Stack is "last in, first out". Think of a stack of plates where you take one off the top
    // Adding data is called "pushing onto the stack"
    // Removing data is called "popping off the stack"
    // All data stored opn the stack must have a known, fixed size.
    // Data with an unknown size at compile time or a size that might change must be sotred on the heap instead.

    //  == Understanding Heap ==
    println!();
    println!("== Understanding Heap ==");
    // Heap is less organized
    // When you put data on the heap, you request a certain amount of space.
    // The memory allocator finds an empty spot in theh heap that is big enough, marks it as being in use, 
    // and returns a "pointer", which is the "address of that location". <- The process is called "allocating" on the heap.
    // pushing values onto the stack is not considered allocating
    // Example of Heap : Think of being seated at a restaurant.
    // when you enter, you state the number of poeple in your group, and the staff finds an empty seats.
    // If someone in your group comes late, they can ask where you've been seated to find you.

    //  == Heap vs Stack ==
    println!();
    println!("== Heap vs Stack ==");
    // pushing to the stack is faster than "allocating" on the heap.
    // Because the allocator never has to search for a place to store new data.
    // That location is always at the top of the stack.
    // Allocating space on the heap requires more work, because the allocator must first find a big enough space to hold data.
    // Then bookkeeping to prepare of the next allocation

    // Accessing data in the heap is slower than accessing data on the stack because it nees to follow a pointer
    // Processors are faster if they jump around less in memory
    // Processor can do its job better if it works on data that's close to other data (as it is on the stack) than far away (heap)
    // Once one understands ownership, one won't need to think about the stack and the heap very often tho

    //  == Ownership Rules ==
    println!();
    println!("== Ownership Rules ==");
    // - Each value in Rust has an owner
    // - There can only be one owner at a time
    // - When the owner goes out of scope, the value will be dropped

    let s = "hello"; // string literal stores data in Stack (immutable)

    // There is a second string type called String.
    // String manages data allocated on the heap and as such is a ble to store an amount of text that is unknown to us at compile time.
    let mut s: String = String::from("hello"); // String stores data in Heap (mutable)
    s.push_str(", world! yo!!"); 
    println!("{}", s);

    // In summary :
    // string literals are fast and efficient but they are immutable
    // string literal's contents are known at compile time

    // String type is mutable and growable piece of text.
    // We need to allocate amount of memory on the heap because it's unknown at compile time
    // - The memory must be requested from the memory allocator at runtime.
    // - We need a way of returning this memory to the allocator when we're done with our String.

    let s1 = String::from("my");
    let s2 = s1; // <- s1 has moved into s2
    println!("{}, name is ninja!", s2);

    //  == Clone ==
    println!();
    println!("== Clone ==");
    let s1 = String::from("clone this");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    // Heap data does get copied
    
    // The integers have a known size at compile time
    // and stored entirely on the stack, so copies of the actual values quick to make.
    // There's no difference between deep and shallow copying here
    // calling clone method wouldn't do anything different
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // Type that implement "Copy" 👇
    // All Integer Types such as u32
    // Boolean Type
    // Floating Type
    // Charater Type like char
    // Tuples if they only contain types that also implement "Copy" (Example ➡️ (i32, i32) can implement "Copy" but (i32, String) can't)

    //  == Heap And Stack's Ownerships and Functions ==
    println!();
    println!("== Heap And Stack's Ownerships and Functions ==");
    let s = String::from("hello");
    takes_ownership(s); // <-- s's value moves into the function ...
    // println!("{}", s); <-- trying to use s after the being moved will throw a compile-time error.

    let x = 5;
    makes_copy(x); // x would move into the functiohn, but i32 is "Copy". so it's ok to still use x afterward

    //  == Return Values and Scope ==
    println!();
    println!("== Return Values and Scope ==");
    let s1 = gives_ownership();
    let s2 = String::from("home");
    let s3 = takes_and_gives_back(s2);
    println!("s1: {}", s1);
    // println!("s2: {}", s2); // <-- This won't work because the ownership moved to s3
    println!("s3: {}", s3);

    //  == Returning Multiple Values ==
    println!();
    println!("== Returning Multiple Values ==");
    let s1 = String::from("home");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    //  == References and Borrowing ==
    println!();
    println!("== References and Borrowing ==");
    let s1 = String::from("Would This Work ?");
    let len = calculate_length_for_reference(&s1); // ampersands represent reference. 
    // ampersands allow you to refer to some value without taking ownership of it.
    println!("The length of '{}' is {}.", s1, len);

    //  == Referencing, aka Borrowking, and Mutating the Data ==
    println!();
    println!("== Referencing, aka Borrowking, and Mutating the Data ==");
    let mut s = String::from("this is mutable "); // declare mutable 
    change(&mut s); // pass mutable reference. the function will mutate the value it borrows.
    println!("{}", s);

    //  == Multiple Mutation Won't Work ==
    println!();
    println!("== Multiple Mutation Won't Work ==");
    let mut s = String::from("home");
    let r1 = &mut s;
    // let r2 = &mut s; // we cannot borrow s as mutable more than once at a time. The first borrow is in r1.
    // Trying to create another mutable reference won't work
    // println!("{}, {}", r1, r2);
    println!("{}", r1 );
    // The benefit of this restriction is that Rust can prevent data races at compile time 👇
    // - Two or more pointers access the same data at the same time.
    // - At least one of the pointers is being used to write to the data.
    // - There’s no mechanism being used to synchronize access to the data.

    //  == To Make Multiple Mutable Reference ==
    println!();
    println!("== To Make Multiple Mutable Reference ==");
    let mut s = String::from("home");
    {
        let r1 = &mut s;
        r1.push_str(" is concatenated, yo this home");
        println!("{r1}");
    }
    let r2 = &mut s;
    println!("{r2}");

    //  == Combining Mutable and Immutable Data ==
    println!();
    println!("These values are from immutable references -> {}, {}", r1, r2);
    println!("== Combining Mutable and Immutable Data ==");
    let mut s = String::from("mutableS");
    let r1 = &s;
    let r2 = &s; //  Multiple immutable references are allowed. Because reading the data doesn't affect the data.
    println!("These values are from immutable references -> {}, {}", r1, r2);
    // let r3 = &mut s; // We can't have a mutable reference while we have immutable reference on same data.
    // println!("{}, {}, {}", r1, r2, r3);
    let r3 = &mut s; // The sceops don't overalp. so this code is allowed.
    println!("{}", r3);

    let reference_to_nothing = dangle();
    println!("{reference_to_nothing}");

    //  == The Slicing ==
    println!();
    println!("== The Slicing ==");
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("{}", word); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""

    //  == String Slices ==
    println!();
    println!("== String Slices ==");
    let s = String::from("hello world");
    let first_word = &s[0..5]; // [index..length]
    let second_word = &s[6..11];
    println!("{}, {}", first_word, second_word);

    let s = String::from("hello");
    let slice = &s[0..2]; // this is equal to 👇
    println!("{slice}");
    let slice = &s[..5];
    println!("{slice}"); // this 👆

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len]; // this is equal to 👇
    println!("{slice}");
    let slice = &s[3..]; // this 👆
    println!("{slice}");

    let s = String::from("hello🔥");
    let len = s.len();
    let slice = &s[0..len]; // this is equal to 👇
    println!("{slice}");
    let slice = &s[..]; // this 👆
    println!("{slice}");
    // String slice only works on UTF-8

    //  == The Slicing Upgrade ==
    println!();
    println!("== The Slicing Upgrade ==");
    let mut s = String::from("hello world");
    let word = first_word_upgrade(&s);
    // s.clear(); // Rust disallows the mutable reference in clear and the immutable reference  in word. Compilation fails.
    println!("{}", word); 
    s.clear(); 

    // let s = "Hello, World!"; // Thus, s is &str. It's slice pointing to the binary. It's immutable.

    //  == Other Slices ==
    println!();
    println!("== Other Slices ==");
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2,3]);

} // Here, x goes out of scope, but s's value has moved

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope and 'drop' is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // some_integer goes out of scope

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String)  -> String {
    a_string
} // When a variable that includes data on the heap goes out of scope, the value will be cleaned up by "drop"
  // unless ownership of the data has been moved to another variable.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_for_reference(s: &String) -> usize {
    println!("when reference is passed : {}", s);
    s.len() // Here, s goes out of scope. but because it doesn't have ownership of what it referto, it's not dropped.
    // when functions have references as parameters like this, we don't need to return the ownership.
    // * We call the action of creating a reference borrowing *
} 

fn change(some_string: &mut String) {
    some_string.push_str("reference, yo");
}

fn dangle() -> String {
    let s = String::from("dangle function from string");
    // &s // Because s i created inside dangle, s will be delocated once the scope ends
    s // The solution is to return the String directly. Ownership is moved out. Nothing is deallocated
}
    // * Note *
    // - You can have either 1 mutabel refernce or ANY number of immutable references
    // - References must always be valid.

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_upgrade(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

    // The concept of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time
    // Having the owner of data automatically clean up that data when the owner goes out of scope
