fn size_label(count: usize) -> &'static str {
    match count {
        0 => "empty",
        1 => "single",
        _ => "multiple",
    }
}

fn main() {
    let count = 3;
    let label = if count == 0 { "empty" } else { "nonempty" };
    assert_eq!(label, "nonempty");
    assert_eq!(size_label(0), "empty");
    assert_eq!(size_label(1), "single");
    assert_eq!(size_label(3), "multiple");
    println!("if: {label}, match: {}", size_label(count));
}
