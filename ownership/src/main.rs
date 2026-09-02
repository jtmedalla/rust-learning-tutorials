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

    // making a copy (use .clone())
    let name = String::from("Bob");
    let name_copy = name.clone();

    dbg!(name);
    dbg!(name_copy);

    // types with known size (i.e. integers) are stored in stack
    let n1 = 100;
    let n2 = n1;

    dbg!(n1, n2);

    // strings are transfered ownership when used as function parameters
    let text = String::from("Bob");
    greet(text);

    // types with known size are copied when used in functions
    let n = 200;
    display_number(n);
    println!("Second attempt at using n : {n}");

    let s1 = create_string();
    let s2 = create_string();
    let s3 = process_text(s1);
    dbg!(s3);


    // transfering ownership via tuple
    let text = String::from("Bob");
    let (text, length) = total_characters(text);

    println!("{text} has a total length of {length}");

    /*
        use the '&' to use the variable's refrence instead.
        values are not moved in memory
        variable can be used for later
    */
    let mut name = String::from("Bob");
    let len = get_length(&name);
    println!("The length of '{name}' is {len}");

    dbg!(&name, &len);

    modify(&mut name);
    println!("Text is {name}");

    let mut text = String::from("Bob");
    /*
        not allowed. can cause data races (race conditions but for data)
        mutable references can be used in different scopes
        let r1 = &mut text;
        let r2 = &mut text;

        also not allowed immutable and mutable reference cannot coexist
        let r1 = &text;
        let r2 = &text;
        let r3 = &mut text;

        you cannot modify the original when there is an active mutable refrence
    */
}

fn greet(name: String) {
    println!("Hello, {name}");
}

fn display_number(n: i32) {
    println!("The number is : {n}");
}

fn create_string() -> String {
    return String::from("Bob");
}

fn process_text(text: String) -> String {
    return text.to_uppercase();
}

fn total_characters(text: String) -> (String, usize) {
    let length = text.chars().count();
    return (text, length);
}

fn get_length(text: &String) -> usize {
    return text.chars().count();
}

// references are immutable by default. add mut keyword
fn modify(text: &mut String) {
    text.push_str("!");
}
