fn decorate(title: String) -> String {
    todo!("Return the owned title with an exclamation mark appended")
}

fn main() {
    let original = String::from("Read Rust");
    // TODO: Make an independent snapshot before transferring original.
    let snapshot: String = todo!("Choose an explicit ownership operation");
    let updated = decorate(original);
    assert_eq!(snapshot, "Read Rust");
    assert_eq!(updated, "Read Rust!");
    println!("snapshot: {snapshot}, updated: {updated}");
}
