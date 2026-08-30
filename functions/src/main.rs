fn print_hello() {
    println!("Hi, Bob!");
    println!("End of function");
}

fn main() {
    print_hello();
    print_hello();
    print_hello();
    goodbye();
}

// functions can be placed before and after main function
fn goodbye() {
    println!("Goodbye, Bob!");
}
