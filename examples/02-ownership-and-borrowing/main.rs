fn count_letters(text: &String) -> usize {
    text.len()
}

fn main() {
    let message = String::from("Rust");
    println!("{} has {} letters", message, count_letters(&message));
    println!("The owner can still use: {message}");
}
