fn main() {
    const LIMIT: usize = 10;
    let mut attempts: usize = 2;
    attempts += 1;
    assert_eq!(attempts, 3);
    assert!(attempts < LIMIT);
    let attempts = attempts.to_string();
    assert_eq!(attempts, "3");
    println!("attempts: {attempts}, limit: {LIMIT}");
}
