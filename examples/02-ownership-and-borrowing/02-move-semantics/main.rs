fn take_ownership(text: String) {
    // `text` is the owner inside this function.
    println!("inside function: {text}");
} // `text` leaves scope, so Rust drops the String here.

fn give_back(text: String) -> String {
    // The function receives ownership and returns it to its caller.
    text
}

fn main() {
    let first = String::from("hello");
    let second = first; // Move the String handle and heap allocation to `second`.

    // `first` cannot be used here: it no longer owns the String.
    println!("after move: {second}");

    take_ownership(second); // Move ownership into the function.

    let returned = give_back(String::from("ownership returned"));
    println!("returned: {returned}");
}
