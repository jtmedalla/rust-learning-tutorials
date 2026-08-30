fn main() {
    check_length("Bobhasahat123");
    check_length("Bob123");

    if long_enough("bob123_5678") {
        println!("Password is long enough!");
    } else {
        println!("Password is too short...");
    }
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
    let length = password.len();

    if length >= 10 {
        return true;
    } else {
        return false;
    }

    // can also be: 
    // return length >= 10;
}