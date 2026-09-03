fn main() {
    let mut sentence = String::from("Bob doesn't care.");
    let first_word = get_first_word(&sentence);
    println!("first_word={:?}", first_word);
    sentence.clear();
}

fn get_first_word(sentence: &String) -> usize {
    let bytes = sentence.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return sentence.len();
}