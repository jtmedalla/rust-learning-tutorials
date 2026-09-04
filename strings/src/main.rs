fn main() {
    let mut sentence = String::from("Bob doesn't care.");
    let first_word = get_first_word(&sentence);
    println!("first_word={:?}", first_word);
    sentence.clear();

    let sentence2 = String::from("Bob loves Chinese food.");

    /*
        use slice notation to get a reference to a portion of a string
        slice data structure stores start and end of
    */ 
    let name = &sentence2[0..3];
    let food = &sentence2[10..17];
    dbg!(name, food);

    // s1 and s2 are the same
    let s1 = &sentence2[..3];
    let s2 = &sentence2[0..3];

    // s3 is sliced from start to end of the string
    let s3 = &sentence2[10..];

    // slice entire string
    let s4 = &sentence[..];

    // slice must end on a valid byte. consider multi byte characters
    let name = String::from("Björn");
    dbg!(&name[..4]);

    let mut sentence = String::from("Holy bananas!");
    let word = get_first_word2(&sentence);
    dbg!(word);

    // string literals as slices. Slice pointing to specific binary
    let name = "Bob";
    dbg!(name);
}


// manual slicing
fn get_first_word(sentence: &String) -> usize {
    let bytes = sentence.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return sentence.len();
}

// using string slices 
fn get_first_word2(sentence: &String) -> &str {
    let bytes = sentence.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &sentence[..i];
        }
    }

    return &sentence[..];
}