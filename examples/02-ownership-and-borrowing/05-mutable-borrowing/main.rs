fn add_one(value: &mut i32) {
    // Dereference the mutable reference to change the owner's value.
    *value += 1;
}

fn add_suffix(text: &mut String) {
    // The function mutates the caller's String without taking ownership.
    text.push_str("!");
}

fn main() {
    let mut score = 41;
    add_one(&mut score); // Temporarily lend exclusive access.
    println!("score: {score}"); // The borrow ended when the call finished.

    let mut message = String::from("Rust");
    add_suffix(&mut message);
    println!("message: {message}");
}
