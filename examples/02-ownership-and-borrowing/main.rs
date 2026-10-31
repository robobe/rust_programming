fn count_letters(text: &str) -> usize {
    // A shared borrow lets this function read without taking ownership.
    text.len()
}

fn main() {
    let message = String::from("Rust");
    // `&message` creates a temporary shared reference to the String.
    println!("{} has {} letters", message, count_letters(&message));
    // Ownership stayed in `main`, so the owner can use the String again.
    println!("The owner can still use: {message}");
}
