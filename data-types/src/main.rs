fn main() {
    let user_input = "100";
    let converted: u32 = user_input.parse().expect("Could not parse");

    println!("converted={}", converted);

    // scalar = represents a single value
    let number: i8 = 10;
    let pi: f32 = 3.1415;
    let turned_on: bool = false;
    let delta: char = 'd';

    // compound = group multiple values
    let coordinates: (f32, f32) = (1.5, 2.5);
    let people: [&str; 3] = ["Bob", "Luigi", "Ashley"];

    // signed integer
    let n1: i8 = -10;   // -128 to 127

    // signed integer
    let n2: u8 = 200;   // 0 to 255

    // for determining max and min value of a datatype (replace "isize")
    // isize determined by architecture of computer
    println!("{}", isize::MAX);
    println!("{}", isize::MIN);

    // integer overflow. error in debug mode, overflow in release mode
    // let mut x: u8 = 255;
    // x += 10;
    // println!("x={}", x);

    // rust ignores underscores (see: formatted)
    let trillion: i64 = 1000000000000;
    let formatted: i64 = 1_000_000_000_000;

    println!("trillion={trillion}");
    println!("formatted={formatted}");

    // floats
    let pi2: f32 = 3.1415927;
    let decimal: f64 = 2.718281828459045;

    println!("pi={pi2}");
    println!("decimal={decimal}");

    // floats cannot be perfectly represented
    let a: f64 = 0.1;
    let b: f64 = 0.2;
    let sum: f64 = a + b;
    println!("sum={sum}");
    println!("sum={}", sum == 0.3);

    // booleans
    let connected_to_internet: bool = false;
    let has_cat: bool = true;
    println!("connected_to_internet={}", connected_to_internet);
    println!("has_cat={}", has_cat);
    
    // in line boolean
    let money: i32 = 5_000;
    println!("money={money}");
    println!("money > 0 = {}", money > 0);

    if money > 0 {
        println!("You are not broke!");
    }

    // character
    let letter: char = 'z';
    let omega: char = 'o';
    let heart: char = '<';
    println!("{} {} {}", letter, omega, heart);
}
