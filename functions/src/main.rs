fn print_hello() {
    println!("Hi, Bob!");
    println!("End of function");
}

// function with parameter
fn hello(name: &str) {
    println!("Hello, {name}!");
}

// function with multiple parameters
fn repeat(text: &str, times: usize) {
    println!("{}", text.repeat(times));
}

// function with return value
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    // "return" and ";" are optional as final line of code
    return celsius * 9.0 / 5.0 + 32.0;
}

fn add(a: i32, b: i32) -> isize {
    println!("Adding {a} + {b}");
    return (a + b) as isize;
}

fn main() {
    // function call
    print_hello();
    print_hello();
    print_hello();

    // function call with parameter
    hello("Bob");
    hello("James");

    // function call with multiple parameters
    repeat("Bob", 3);
    repeat("Z", 10);

    // function calls with return value
    println!("celsius_to_fahrenheit(20.0)={:?}", celsius_to_fahrenheit(20.0));

    let converted = celsius_to_fahrenheit(10.0);
    println!("{converted}");

    let result = add(10, 20);
    dbg!(result);

    goodbye();
}

// functions can be placed before and after main function
fn goodbye() {
    println!("Goodbye, Bob!");
}
