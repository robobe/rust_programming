fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn main() {
    let sentence = String::from("Rust makes borrowing useful");
    let word = first_word(&sentence);
    assert_eq!(word, "Rust");
    println!("first word: {word}");
    println!("still owned: {sentence}");

    assert_eq!(first_word("hello"), "hello");
    assert_eq!(first_word(""), "");
    assert_eq!(first_word(" \t\n"), "");
    assert_eq!(first_word("  café Rust"), "café");
}
