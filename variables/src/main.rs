const PI: f64 = 3.1415;

fn main() {
    let mut number = 10;
    println!("number = {number}");

    number += 1;
    println!("number = {number}");

    const ONE_MINUTE: i32 = 60;
    const ONE_HOUR: i32 = ONE_MINUTE * 60;

    println!("One minute is: {ONE_MINUTE}s");
    println!("One hour is: {ONE_HOUR}s");

    println!("pi is {PI}");
}
