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

    // while loops
    let mut n = 10;
    while n > 0 {
        n -= 1;

        if n == 5 {
            println!("Skipping 5!");
            // skips an iteration of the loop
            continue;
        }

        // not executed when continue is encountered
        println!("n = {:?}", n);
    }

    // for loops: allow for iteration in iterables easily
    let names = ["Bob", "Ben", "Betty"];

    for name in names {
        println!("{name} says: hi!");
    }

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let mut power_total = 0;

    for number in numbers {
        let squared = number.pow(2);
        println!("{number}: {:?}", squared);
        power_total += squared;
    }

    dbg!(power_total);

    // better to use for loop here vs while
    let names = ["Bob", "Ben", "Betty"];
    let mut index = 0;

    while index < names.len() {
        dbg!(names[index]);
        index += 1;
    }

    // loop labels
    let mut main_count = 0;

    'main: loop {
        println!("Outer: {main_count}");
        let mut inner_count = 0;

        loop {
            println!("Inner: {inner_count}");
            inner_count += 1;

            if inner_count == 3 {
                println!("---");
                break;
            }

            if main_count == 3 {
                println!("Exiting out of all loops");
                break 'main;
            }
        }

        main_count += 1;
    }
}
