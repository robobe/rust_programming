fn first_word(text: &str) -> &str {
    // Return a view into the caller's text without allocating or taking ownership.
    text.split_whitespace().next().unwrap_or("")
}

fn sum_values(values: &[i32]) -> i32 {
    // A slice accepts a Vec, an array, or part of either collection.
    values.iter().sum()
}

fn print_text<T: AsRef<str>>(text: T) {
    // AsRef lets this read from several string-like input types.
    println!("text: {}", text.as_ref());
}

fn main() {
    let sentence = String::from("Rust makes views useful");
    let first = first_word(&sentence);
    println!("first word: {first}");

    let numbers = [1, 2, 3, 4];
    println!("array sum: {}", sum_values(&numbers));
    println!("middle sum: {}", sum_values(&numbers[1..3]));

    let owned_numbers = vec![5, 6, 7];
    println!("Vec sum: {}", sum_values(&owned_numbers));

    print_text("a literal");
    print_text(sentence.as_str());
}
