fn func() {
    // s is only usable inside func
    let s = "Hello, World!";
    println!("s={:?}", s);
}

fn main() {
    // stack: last-in-first-out
    // heap: less organized than stack. Memory organizer allocates memory then returns pointer
    // scope: range within a program where a variable is valid


    /* 
        string literals (string slice). Cannot be modified (immutable)
        Strings are stored in the stack. Hardcoded directly in executable (fast and efficicent)
    */
    let wisdom = "Wash your hands with soap";
    dbg!(wisdom);

    // Rust knows text is a string with 3 characters
    let text = "Bob";

    // String type
    let mut s = String::from("Hello");
    s += ", Bob!";
    println!("s={:?}", s);

    /*
        To mutable string:
            Needs to request memory to allocator (declaration)
            Need to return memory after use (after variable goes out of scope)
     */
    {
        let text = String::from("Bob");
    }


    /*
        String is made up of 3 parts:
            1. pointer to memory that holds contents
            2. length - how much memory in bytes contents of string is using
            3. capacity - total amt of memory in bytes that memory allocated string
        Stored in stack

        Rust automatically drops the original reference when a new one is used
        Also known as "move"
     */
    let original_text = String::from("Bob");
    let name = original_text;

    dbg!(name);

    
}

