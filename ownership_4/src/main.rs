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

    let s: &str= "hello"; // string literal stores data in Stack (immutable)

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

}
