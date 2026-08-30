fn main() {
    check_length("Bobhasahat123");
    check_length("Bob123");

    if long_enough("bob123_5678") {
        println!("Password is long enough!");
    } else {
        println!("Password is too short...");
    }

    dbg!(get_response("Hello, Bob!"));
    dbg!(get_response("How are you?"));
    dbg!(get_response("Good"));
    dbg!(get_response("Is this a cat in a hat?"));

    analyse_number(100);

    let n = 10;
    // values to return must be of same type
    let odd_even = if n % 2 == 0 {"Even"} else {"Odd"};
    dbg!(odd_even);
}

fn check_length(password: &str) {
    let length = password.len();

    if length >= 10 {
        println!("'{password}' is long enough!");
    } else {
        println!("'{password}' is NOT long enough! Please add more characters...");
    }
}

fn long_enough(password: &str) -> bool {
    let length = password.chars().count();

    if length >= 10 {
        return true;
    } else {
        return false;
    }

    // can also be: 
    // return length >= 10;
}

fn get_response(input: &str) -> &str {
    let lowered: String = input.to_lowercase();

    // order matters. first evaluated is the only one that counts
    if lowered.contains("hello") {
        return "Hello there!";
    } else if lowered.contains("how are you") {
        return "Good, and you";
    } else if lowered.contains("good") {
        return "Good is good.";
    } else {
        return "I don't understand";
    }
}

fn analyse_number(n: i32) {
    if n > 0 {
        println!("{n} is greater than 0!");
    } else if n > 10 {
        println!("{n} is greater than 10!");
    } else {
        println!("{n} is a cool number!");
    }
}