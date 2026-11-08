fn decorate(mut title: String) -> String {
    title.push('!');
    title
}

fn main() {
    let original = String::from("Read Rust");
    let snapshot = original.clone();
    let updated = decorate(original);
    assert_eq!(snapshot, "Read Rust");
    assert_eq!(updated, "Read Rust!");
    println!("snapshot: {snapshot}, updated: {updated}");

    let original = String::new();
    let snapshot = original.clone();
    let updated = decorate(original);
    assert_eq!(snapshot, "");
    assert_eq!(updated, "!");
}
