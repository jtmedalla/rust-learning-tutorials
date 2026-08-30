fn main() {
    // statements do not remove values
    let name = "Bob";
    println!("Hello, {name}");

    // statements do not return code
    // let var = (let var2 = 1);

    // expressions return values
    dbg!(20 + 50);

    // blocks return values
    let sum = {
        let x = 10;
        let y = 20;
        
        x + y
    };
}
