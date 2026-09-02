fn main() {
    // for infinite loops
    let mut counter = 0;
    let result = loop {
        println!("Count: {counter}");
        counter += 1;

        if counter == 5 {
            // break keyword terminates the infinite loop
            // stuff after break returns after loop terminates
            break "SUCCESS";
        }
    };
    dbg!(result);
}
